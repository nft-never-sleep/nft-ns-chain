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
* 借品NFT  
    * 通过在metadata的description字段填入NFT统一ID，建立起借品与原件之间的链上关系;
    * 在metadata的title字段填入NFT Never Sleep，标识此为借品NFT;
    * 通过metadata的issue_at, start_at, expire_at字段描述借品的时间范围;

