
use crate::*;
use crate::utils::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum BidState {
    InProgress,
    Approved,
    Rejected,
    Expired,
    Consumed,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct Bid {
    pub src_nft_id: String,
    pub origin_owner: String,
    pub start_at: u32,
    pub lasts: u32,
    pub amount: Balance,
    pub msg: String,
    pub bid_from: AccountId,
    pub bid_state: BidState,
    pub create_at: Timestamp,
}

impl From<&BidInfo> for Bid {
    fn from(bid: &BidInfo) -> Self {
        Self {
            src_nft_id: bid.src_nft_id.clone(),
            origin_owner: "".to_string(),
            start_at: bid.start_at,
            lasts: bid.lasts,
            amount: bid.amount.into(),
            msg: bid.msg.clone(),
            bid_from: bid.bid_from.clone(),
            bid_state: BidState::InProgress,
            create_at: env::block_timestamp(),
        }
    }
}

impl Bid {
    pub fn cur_state(&self, expire_sec: u32) -> BidState {
        match self.bid_state {
            BidState::InProgress => {
                if self.create_at + sec_to_nano(expire_sec) < env::block_timestamp() {
                    BidState::Expired
                } else {
                    BidState::InProgress
                }
            }
            _ => { self.bid_state.clone() }
        }
    }

    pub fn accept_bid(&mut self, opinion: bool, expire_sec: u32) {
        self.bid_state = self.cur_state(expire_sec);
        match self.bid_state {
            BidState::InProgress => {
                if opinion {
                    self.bid_state = BidState::Approved;
                } else {
                    self.bid_state = BidState::Rejected;
                }
            }
            _ => {}
        }
    }
}

impl Contract {
    pub(crate) fn internal_add_bid(&mut self, bid_info: &BidInfo) -> u64 {
        let bid: Bid = bid_info.into();
        let id = self.bids.len() as u64;
        self.bids.push(&bid);
        id
    }

    pub(crate) fn internal_take_offer(&mut self, owner_id: &AccountId, bid_id: u64, opinion: bool) {
        let mut bid = self.bids.get(bid_id).expect("ERR_NO_BID");
        bid.accept_bid(opinion, self.bid_expire_sec);
        bid.origin_owner = owner_id.clone();
        self.bids.replace(bid_id, &bid);
    }
}
