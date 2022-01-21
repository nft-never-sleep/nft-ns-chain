use near_sdk_sim::{call, view, to_yocto};
use near_sdk::json_types::U128;
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;

use nns::{BidInfo, BidState};

mod common;
use crate::common::{
    init::*,
    utils::*,
};

#[test]
fn test_bid() {
    let (root, owner, user, testnft_contract, nns_contract) = 
        init_env();
    
    let out_come = call!(
        owner,
        testnft_contract.mint_nft("fake_nft_1".to_string(), user.valid_account_id(), Some(TokenMetadata {
            title: Some("fake_nft_1".to_string()),          
            description: Some("Fake NFT #01".to_string()),  
            media: None, 
            media_hash: None, 
            copies: None, 
            issued_at: None, 
            expires_at: None, 
            starts_at: None, 
            updated_at: None, 
            extra: None, 
            reference: None, 
            reference_hash: None, 
        })),
        deposit = 0
    );
    // println!("{:#?}", out_come.promise_results());
    out_come.assert_success();

    let out_come = call!(
        owner,
        testnft_contract.mint_nft("fake_nft_2".to_string(), user.valid_account_id(), Some(TokenMetadata {
            title: Some("fake_nft_2".to_string()),          
            description: Some("Fake NFT #02".to_string()),  
            media: None, 
            media_hash: None, 
            copies: None, 
            issued_at: None, 
            expires_at: None, 
            starts_at: None, 
            updated_at: None, 
            extra: None, 
            reference: None, 
            reference_hash: None, 
        })),
        deposit = 0
    );
    out_come.assert_success();

    let nfts = testnft_tokens_for_owner(&testnft_contract, user.valid_account_id());
    // println!("{:#?}", nfts);
    println!("count: {}", nfts.len());

    let nft = testnft_token(&testnft_contract, "fake_nft_1".to_string());
    // println!("nft: {:#?}", nft);

    let user1 = root.create_user("user1".to_string(), to_yocto("100"));

    let out_come = call!(
        user1,
        nns_contract.offer_bid(BidInfo {
            src_nft_id: "test_nft:fake_nft_1".to_string(),
            orgin_owner: "".to_string(),
            start_at: 100,
            lasts: 60,
            amount: U128(to_yocto("5")),
            msg: "".to_string(),
            bid_from: user1.account_id(),
            bid_state: None,
        }),
        deposit = to_yocto("1")
    );
    // println!("{:#?}", out_come.promise_results());
    out_come.assert_success();

    let bids = nns_list_bids_by_sender(&nns_contract, user1.valid_account_id());
    assert_eq!(bids.get(&0).unwrap().bid_from, "user1".to_string());
    let bids = nns_list_bids_by_nft(&nns_contract, "test_nft:fake_nft_1".to_string());
    assert_eq!(bids.get(&0).unwrap().src_nft_id, "test_nft:fake_nft_1".to_string());

    let out_come = call!(
        user,
        nns_contract.take_offer(0, true),
        deposit = 1
    );
    // println!("{:#?}", out_come.promise_results());
    out_come.assert_success();

    let bid = nns_get_bid(&nns_contract, 0).unwrap();
    assert_eq!(bid.bid_state, Some(BidState::Approved));

    let out_come = call!(
        user1,
        nns_contract.claim_nft(0),
        deposit = to_yocto("6")
    );
    // println!("{:#?}", out_come.promise_results());
    out_come.assert_success();

    let bid = nns_get_bid(&nns_contract, 0).unwrap();
    assert_eq!(bid.bid_state, Some(BidState::Consumed));
    let nft = nns_token(&nns_contract, "0".to_string());
    assert_eq!(nft.unwrap().owner_id, "user1".to_string());

    let nfts = nns_tokens_for_owner(&nns_contract, user1.valid_account_id());
    println!("{:#?}", nfts);
    println!("count: {}", nfts.len());
}
