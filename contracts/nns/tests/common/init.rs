use near_sdk_sim::{call, deploy, init_simulator, to_yocto, ContractAccount, UserAccount};
use test_nft::ContractContract as TestNft;
use nns::ContractContract as Nns;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    TESTNFT_WASM_BYTES => "../res/test_nft.wasm",
    NNS_WASM_BYTES => "../res/nns.wasm",
}

pub fn init_env() -> (UserAccount, UserAccount, UserAccount, ContractAccount<TestNft>, ContractAccount<Nns>){
    let root = init_simulator(None);

    let owner = root.create_user("owner".to_string(), to_yocto("100"));
    let user = root.create_user("user".to_string(), to_yocto("100"));

    let nft_contract = deploy!(
        contract: TestNft,
        contract_id: "test_nft",
        bytes: &TESTNFT_WASM_BYTES,
        signer_account: root
    );
    call!(root, nft_contract.new(owner.valid_account_id())).assert_success();

    let nns_contract = deploy!(
        contract: Nns,
        contract_id: "nns",
        bytes: &NNS_WASM_BYTES,
        signer_account: root
    );
    call!(root, nns_contract.new(owner.valid_account_id())).assert_success();
    
    (root, owner, user, nft_contract, nns_contract)
}
