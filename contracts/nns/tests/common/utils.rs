use std::collections::HashMap;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::serde_json::{Value, from_value};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk_sim::{
    view, ContractAccount, ExecutionResult,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use nns::{ContractContract as Nns, BidInfo};
use test_nft::ContractContract as TestNft;


pub fn get_error_count(r: &ExecutionResult) -> u32 {
    r.promise_errors().len() as u32
}

pub fn get_error_status(r: &ExecutionResult) -> String {
    format!("{:?}", r.promise_errors()[0].as_ref().unwrap().status())
}

//*****************************
// View functions
//*****************************

pub fn testnft_tokens_for_owner(
    nft: &ContractAccount<TestNft>, 
    account_id: ValidAccountId
) -> Vec<Token> {
    view!(nft.nft_tokens_for_owner(account_id, None, None)).unwrap_json::<Vec<Token>>()
}

pub fn testnft_token(
    nft: &ContractAccount<TestNft>, 
    token_id: String
) -> Option<Token> {
    view!(nft.nft_token(token_id)).unwrap_json::<Option<Token>>()
}

pub fn nns_tokens_for_owner(
    nns: &ContractAccount<Nns>, 
    account_id: ValidAccountId
) -> Vec<Token> {
    view!(nns.nft_tokens_for_owner(account_id, None, None)).unwrap_json::<Vec<Token>>()
}

pub fn nns_token(
    nns: &ContractAccount<Nns>, 
    token_id: String
) -> Option<Token> {
    view!(nns.nft_token(token_id)).unwrap_json::<Option<Token>>()
}

pub fn nns_list_bids_by_sender(
    nns: &ContractAccount<Nns>, 
    account_id: ValidAccountId
) -> HashMap<u64, BidInfo> {
    view!(nns.list_bids_by_sender(account_id)).unwrap_json::<HashMap<u64, BidInfo>>()
}

pub fn nns_list_bids_by_nft(
    nns: &ContractAccount<Nns>, 
    nft_id: String
) -> HashMap<u64, BidInfo> {
    view!(nns.list_bids_by_nft(nft_id)).unwrap_json::<HashMap<u64, BidInfo>>()
}

pub fn nns_get_bid(
    nns: &ContractAccount<Nns>, 
    bid_id: u64
) -> Option<BidInfo> {
    view!(nns.get_bid(bid_id)).unwrap_json::<Option<BidInfo>>()
}
