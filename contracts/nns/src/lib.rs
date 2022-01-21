/*!
* NFT Never Sleep contract
*
*/
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::{LookupMap, LazyOption, Vector, UnorderedSet};
use near_sdk::{assert_one_yocto, env, near_bindgen, AccountId, Balance, BorshStorageKey, 
    ext_contract, PanicOnDefault, Promise, PromiseResult, PromiseOrValue, Timestamp, CryptoHash};

use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};

use crate::borrower::Borrower;
use crate::nft_owner::Subject;
pub use crate::bid::{Bid, BidState};
pub use crate::view::BidInfo;
use crate::utils::DATA_IMAGE_SVG_PARAS_ICON;

mod utils;
mod bid;
mod nft_owner;
mod borrower;
mod view;

near_sdk::setup_alloc!();

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    TokensPerOwner { account_hash: Vec<u8> },
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,

    Bids,
    Borrowers,
    Subjects,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    token_num: u32,

    bids: Vector<Bid>,
    bid_expire_sec: u32,
    borrowers: LookupMap<AccountId, Borrower>,
    subjects: LookupMap<String, Subject>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Contract {
            owner_id: owner_id.clone().into(),

            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(
                &NFTContractMetadata {
                    spec: NFT_METADATA_SPEC.to_string(),
                    name: "Nft Never Sleep".to_string(),
                    symbol: "nns".to_string(),
                    icon: Some(DATA_IMAGE_SVG_PARAS_ICON.to_string()),
                    base_uri: None,
                    reference: None,
                    reference_hash: None,
                })),

            token_num: 0,
            bids: Vector::new(StorageKey::Bids),
            bid_expire_sec: 3600 * 24 * 7,
            borrowers: LookupMap::new(StorageKey::Borrowers),
            subjects: LookupMap::new(StorageKey::Subjects),
        }
    }
}

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

impl Contract {
    pub fn internal_mint(&mut self, 
        token_id: TokenId,
        token_owner_id: ValidAccountId,
        token_metadata: Option<TokenMetadata>,
    ) -> Token {
        if self.tokens.token_metadata_by_id.is_some() && token_metadata.is_none() {
            env::panic(b"Must provide metadata");
        }
        if self.tokens.owner_by_id.get(&token_id).is_some() {
            env::panic(b"token_id must be unique");
        }

        let owner_id: AccountId = token_owner_id.into();

        // Core behavior: every token must have an owner
        self.tokens.owner_by_id.insert(&token_id, &owner_id);

        // Metadata extension: Save metadata, keep variable around to return later.
        // Note that check above already panicked if metadata extension in use but no metadata
        // provided to call.
        self.tokens.token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &token_metadata.as_ref().unwrap()));

        // Enumeration extension: Record tokens_per_owner for use with enumeration view methods.
        if let Some(tokens_per_owner) = &mut self.tokens.tokens_per_owner {
            let mut token_ids = tokens_per_owner.get(&owner_id).unwrap_or_else(|| {
                UnorderedSet::new(StorageKey::TokensPerOwner {
                    account_hash: env::sha256(owner_id.as_bytes()),
                })
            });
            token_ids.insert(&token_id);
            tokens_per_owner.insert(&owner_id, &token_ids);
        }

        Token { token_id, owner_id, metadata: token_metadata, approved_account_ids: None }
    }
}

