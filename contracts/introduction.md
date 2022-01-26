# NFT-NS 合约介绍
## 合约职责
* 为租借双方提供报价和协商的无需信任的渠道
* 无需信任的租借流程
## 合约特点
* 最简化租借流程  
    租借双方三步完成租借过程：
    * offer_bid borrower发起报价请求;
    * take_offer nft owner响应报价;
    * claim_nft borrower执行租约;

* NFT统一ID  
    通过[contract_id]:[Token_ID]的模式，统一标记near生态上的所有NFT；
* 全生命周期Bid报价设计  
    ```rust
    pub enum BidState {
        InProgress,
        // nft owner accept the bid
        Approved,
        // nft owner explicit reject the bid
        Rejected,
        Expired,
        // nft borrower execute the lease
        Consumed,
    }
    pub struct Bid {
        // global NFT id
        pub src_nft_id: String,
        // nft owner, verified on chain
        pub origin_owner: String,
        // start timestamp of the lease
        pub start_at: u32,
        // duration in seconds of the lease
        pub lasts: u32,
        // total fee of the lease
        pub amount: Balance,
        // extra negotiation info
        pub msg: String,
        // bid creator, that is the borrower
        pub bid_from: AccountId,
        pub bid_state: BidState,
        // bid creation time, used to tell expiration
        pub create_at: Timestamp,
    }
    ```
    * 报价有过期机制，NFT所有者无需主动拒绝不合适的报价；
    * 报价有保证金机制，创建报价时需支付1Near的押金（可通过合约所有者，一般是DAO，进行修改），防止flood bid attack;
    * 得益于near的NFT标准NEP171/177，以及优异的cross contract call设计，nft的owner由链上负责验证;
    * 租期时间颗粒度细化到秒级;

* 原子化的租借过程  
    当一份租借bid被nft owner接受后，borrower通过一个合约调用完成全部租借动作，包括：铸造借品NFT，支付租借费给nft owner，退还报价押金。

    其中在owner调用`take_offer`合约接口响应`bid`的时候，合约通过`cross contract call`获取nft的链上真实owner进行验证，保证调用者为nft所有者:
    ```rust
    ext_contract::nft_token(
        token_id.to_string(),
        &token_contract.to_string(),
        0,
        GAS_FOR_VIEW,
    )
    .then(ext_self::on_nft_token_callback(
        caller_id.clone(),
        bid_id,
        opinion,
        &env::current_account_id(),
        0,
        GAS_FOR_RESOLVE,
    ))
    ```

    ```rust
    #[private]
    pub fn on_nft_token_callback(&mut self, caller: AccountId, bid_id: u64, opinion: bool) {
        ...
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {}
            PromiseResult::Successful(value) => {
                let parsed_result = near_sdk::serde_json::from_slice::<Token>(&value);
                if parsed_result.is_ok() {
                    let nft_owner = parsed_result.ok().and_then(|token| Some(token.owner_id)).unwrap();
                    env::log(
                        format!(
                            "Checking result --> nft owner: {}, caller: {}",
                            nft_owner.clone(), caller.clone(),
                        )
                        .as_bytes(),
                    );
                    assert_eq!(nft_owner, caller, "Err: only owner of the NFT can take offer");
                    self.internal_take_offer(&caller, bid_id, opinion);
                }
            }
        }
    }
    ```


* 借品NFT  
    * 通过在metadata的description字段填入NFT统一ID，建立起借品与原件之间的链上关系;
    * 在metadata的title字段填入NFT Never Sleep，标识此为借品NFT;
    * 通过metadata的issue_at, start_at, expire_at字段描述借品的时间范围;

