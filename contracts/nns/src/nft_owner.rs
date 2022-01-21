use crate::*;
use crate::utils::*;


#[derive(BorshSerialize, BorshDeserialize)]
pub struct Subject {
    pub bids: Vec<u64>,
}

impl Subject {
    pub fn new() -> Self {
        Subject {
            bids: vec![],
        }
    }
}

impl Contract {
    pub fn internal_get_subject(&self, nft_id: &String) -> Option<Subject> {
        self.subjects
            .get(nft_id)
    }

    pub(crate) fn internal_save_subject(&mut self, nft_id: &String, subject: &Subject) {
        self.subjects.insert(nft_id, subject);
    }
}

#[ext_contract(ext_contract)]
trait ExtContract {
    fn nft_token(&mut self, token_id: TokenId);
}

#[ext_contract(ext_self)]
trait ExtSelf {
    fn on_nft_token_callback(&mut self, caller: AccountId, bid_id: u64, opinion: bool);
}

#[near_bindgen]
impl Contract {
    /// nft owner call this to add finalized state into bid
    #[payable]
    pub fn take_offer(&mut self, bid_id: u64, opinion: bool) -> Promise {
        assert_one_yocto();

        let caller_id = env::predecessor_account_id();

        let bid = self.bids.get(bid_id).expect("ERR_NO_BID");

        let (token_contract, token_id) = {
            let pos = bid.src_nft_id.find(":").unwrap_or(bid.src_nft_id.len());
            let (token_contract, remains) = bid.src_nft_id.split_at(pos);
            let (_, token_id) = remains.split_at(1);
            (token_contract, token_id)
        };

        env::log(
            format!(
                "Checking token {} on {}",
                token_id.to_string(), token_contract.to_string(),
            )
            .as_bytes(),
        );

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
    }

    #[private]
    pub fn on_nft_token_callback(&mut self, caller: AccountId, bid_id: u64, opinion: bool) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Err: expected 1 promise result from nft_token"
        );
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
}
