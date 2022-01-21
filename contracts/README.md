# nft_never_sleep

### NFT Standard Interface (NEP-171/177/181)
Some structures are defined in [NEP-177](https://nomicon.io/Standards/NonFungibleToken/Metadata.html)
```rust
/// approved_account_ids not used in this contract
pub struct Token {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: Option<TokenMetadata>,
    pub approved_account_ids: Option<HashMap<AccountId, u64>>,
}
```

#### nft_transfer
```rust
/// 1 yoctoNEAR needed
fn nft_transfer(
    &mut self,
    receiver_id: ValidAccountId,
    token_id: TokenId,
    approval_id: Option<u64>,
    memo: Option<String>,
);
```

#### nft_transfer_call
```rust
/// 1 yoctoNEAR needed
fn nft_transfer_call(
    &mut self,
    receiver_id: ValidAccountId,
    token_id: TokenId,
    approval_id: Option<u64>,
    memo: Option<String>,
    msg: String,
) -> PromiseOrValue<bool>;
```

#### nft_metadata
``rust
fn nft_metadata(&self) -> NFTContractMetadata;
```

#### nft_token
```rust
fn nft_token(self, token_id: TokenId) -> Option<Token>;
```

#### nft_total_supply
```rust
fn nft_total_supply(self) -> U128;
```

#### nft_tokens
```rust
fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token>;
```

#### nft_supply_for_owner
```rust
fn nft_supply_for_owner(self, account_id: ValidAccountId) -> U128;
```

#### nft_tokens_for_owner
```rust
fn nft_tokens_for_owner(
    &self,
    account_id: ValidAccountId,
    from_index: Option<U128>,
    limit: Option<u64>,
) -> Vec<Token>;
```

### Custom Interface

```rust
pub enum BidState {
    InProgress,
    Approved,
    Rejected,
    Expired,
}

pub struct BidInfo {
    /// the source nft id, with format contract_id:nft_id
    pub src_nft_id: String,
    /// the owner of source nft
    pub orgin_owner: String,
    /// the start date of lease, timestamp in sec 
    pub start_at: u32,
    /// the time duration of the lease in sec
    pub lasts: u32,
    /// total lease fee in NEAR
    pub amount: U128,
    /// other infomation from the borrower
    pub msg: String,
    /// the borrower account id
    pub bid_from: AccountId,
    /// see enum BidState
    pub bid_state: Option<BidState>,
}
```
#### offer_bid
borrower call this to request a lease, and deposit a fixed amount of NEAR as bid endorsement: 
```rust
/// nft_id: the one want to lease
/// bid_info: lease details, the bid_state set to None
/// return bid id
pub fn offer_bid(&mut self, nft_id: String, bid_info: BidInfo) -> u64;
```

#### take_offer
owner call this to respond a lease request:
```rust
/// bid_id: id of the bid
/// opinion: true means approve, false means reject
/// need 1 yocto NEAR for secure reason
pub fn take_offer(&mut self, bid_id: u64, opinion: bool) -> Promise;
```

#### claim_nft
borrower call this to claim lease NFT on an approved bid.
borrower should deposit more than amount in approved bid, remaining would be refunded.
```rust
/// bid_id: id of the bid
pub fn claim_nft(&mut self, bid_id: u64);
```

#### get_bid

```rust
/// return None or BidInfo
pub fn get_bid(bid_id: u64) -> Option<BidInfo>;
```

#### list_bids_by_sender
```rust
/// sender_id, bider account ID
/// return HashMap <bid_id, BidInfo>
pub fn list_bids_by_sender(sender_id: ValidAccountId) -> HashMap<u64, BidInfo>;
```

#### list_bids_by_nft
```rust
/// src_nft_id, the one in BidInfo
/// return HashMap <bid_id, BidInfo>
pub fn list_bids_by_nft(src_nft_id: String) -> HashMap<u64, BidInfo>;
```