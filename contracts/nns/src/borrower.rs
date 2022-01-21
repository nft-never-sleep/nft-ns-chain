use std::convert::TryInto;

use crate::*;
use crate::utils::*;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Borrower {
    pub bids: Vec<u64>,
}

impl Borrower {
    pub fn new() -> Self {
        Borrower {
            bids: vec![],
        }
    }
}

impl Contract {
    pub fn internal_get_borrower(&self, account_id: &AccountId) -> Option<Borrower> {
        self.borrowers
            .get(account_id)
    }

    pub(crate) fn internal_save_borrower(&mut self, account_id: &AccountId, borrower: &Borrower) {
        self.borrowers.insert(account_id, borrower);
    }
}

#[near_bindgen]
impl Contract {
    /// borrower deposit 1 near as endorsement to create a new bid
    /// nft owner implies in nft_id
    /// bid valid period is global config
    #[payable]
    pub fn offer_bid(&mut self, bid_info: BidInfo) -> u64 {
        assert_eq!(env::attached_deposit(), ONE_NEAR, "");
        let sender_id = env::predecessor_account_id();
        let nft_id = bid_info.src_nft_id.clone();

        let id = self.internal_add_bid(&bid_info);

        let mut borrower = self.internal_get_borrower(&sender_id)
            .unwrap_or_else(|| Borrower::new());
        borrower.bids.push(id);
        self.internal_save_borrower(&sender_id, &borrower);

        let mut subject = self.internal_get_subject(&nft_id)
            .unwrap_or_else(|| Subject::new());
        subject.bids.push(id);
        self.internal_save_subject(&nft_id, &subject);

        id
    }

    /// nft borrower call this and  deposit bid amount of near
    /// trigger process:
    ///   mint_nft: mint new nns nft belong to borrower with correct metadata.
    #[payable]
    pub fn claim_nft(&mut self, bid_id: u64) -> Token {

        let sender_id = env::predecessor_account_id();

        let mut bid = self.bids.get(bid_id).expect("ERR_NO_BID");
        
        // check bid is valid to mint nft
        assert_eq!(bid.bid_state, BidState::Approved, "ERR_INVALID_BID");
        let amount = env::attached_deposit();
        assert!(amount >= bid.amount, "ERR_INSURFFICIENT_AMOUNT");
        bid.bid_state = BidState::Consumed;
        self.bids.replace(bid_id, &bid);


        let token_id = format!("{}", self.token_num);
        
        let metadata = Some(TokenMetadata {
            title: Some("NFT NEVER SLEEP".to_string()),          // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
            description: Some(bid.src_nft_id.clone()),    // free-form description
            media: None, // URL to associated media, preferably to decentralized, content-addressed storage
            media_hash: None, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
            copies: None, // number of copies of this set of metadata in existence when token was minted.
            issued_at: Some(env::block_timestamp().to_string()), // ISO 8601 datetime when token was issued or minted
            expires_at: Some(sec_to_nano(bid.start_at + bid.lasts).to_string()), // ISO 8601 datetime when token expires
            starts_at: Some(sec_to_nano(bid.start_at).to_string()), // ISO 8601 datetime when token starts being valid
            updated_at: None, // ISO 8601 datetime when token was last updated
            extra: None, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
            reference: None, // URL to an off-chain JSON file with more info.
            reference_hash: None, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
        });

        self.token_num += 1;
        
        let token = self.internal_mint(token_id, sender_id.clone().try_into().unwrap(), metadata);

        // send near to owner and remain back to caller
        Promise::new(bid.origin_owner).transfer(bid.amount);
        // at least refund 1 near bid endorsement fee
        let refund = ONE_NEAR + amount - bid.amount;
        Promise::new(sender_id).transfer(refund);

        token
    }

}
