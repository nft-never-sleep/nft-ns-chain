//! View functions for the contract.

use std::collections::HashMap;

use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId};
use crate::*;

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(PartialEq, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct BidInfo {
    pub src_nft_id: String,
    pub orgin_owner: String,
    pub start_at: u32,
    pub lasts: u32,
    pub amount: U128,
    pub msg: String,
    pub bid_from: AccountId,
    pub bid_state: Option<BidState>,
}

impl From<&Bid> for BidInfo {
    fn from(bid: &Bid) -> Self {
        Self {
            src_nft_id: bid.src_nft_id.clone(),
            orgin_owner: bid.origin_owner.clone(),
            start_at: bid.start_at,
            lasts: bid.lasts,
            amount: bid.amount.into(),
            msg: bid.msg.clone(),
            bid_from: bid.bid_from.clone(),
            bid_state: Some(bid.bid_state.clone()),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_bid(&self, bid_id: u64) -> Option<BidInfo> {
        if let Some(bid) = self.bids.get(bid_id) {
            Some((&bid).into())
        } else {
            None
        }
    }

    pub fn list_bids_by_sender(&self, sender_id: ValidAccountId) -> HashMap<u64, BidInfo> {
        self.internal_get_borrower(sender_id.as_ref())
        .unwrap_or(Borrower::new())
        .bids
        .iter()
        .map(|bid_id| (*bid_id, self.get_bid(*bid_id).unwrap()))
        .collect()
    }

    pub fn list_bids_by_nft(&self, nft_id: String) -> HashMap<u64, BidInfo> {
        self.internal_get_subject(&nft_id)
        .unwrap_or(Subject::new())
        .bids
        .iter()
        .map(|bid_id| (*bid_id, self.get_bid(*bid_id).unwrap()))
        .collect()
    }
}