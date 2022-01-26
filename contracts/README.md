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
#### offer_bid
borrower call this to request a lease, and deposit a fixed amount of NEAR as bid endorsement: 
```rust
/// nft_id: the nft want to lease
/// bid_info: lease details, the bid_state set to None
/// return bid id
#[payable]
pub fn offer_bid(&mut self, nft_id: String, bid_info: BidInfo) -> u64;
```

#### take_offer
owner call this to respond a lease request:
```rust
/// bid_id: id of the bid
/// opinion: true means approve, false means reject
/// need 1 yocto NEAR for secure reason
#[payable]
pub fn take_offer(&mut self, bid_id: u64, opinion: bool) -> Promise
```

#### claim_nft
borrower call this to claim lease NFT on an approved bid.
borrower should deposit more than amount in approved bid, remaining would be refunded.
```rust
/// bid_id: id of the bid
#[payable]
pub fn claim_nft(&mut self, bid_id: u64) -> Token;
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