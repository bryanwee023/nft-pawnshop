use std::convert::TryInto;

use near_sdk::{borsh::BorshSerialize, test_utils::{VMContextBuilder, accounts}, AccountId, json_types::ValidAccountId, env};
use near_sdk::MockedBlockchain;
use near_sdk::testing_env;

use crate::{test_utils::*, pawn::Pawn, Contract};

fn get_context(predecessor_account_id: AccountId, deposit: u128) -> VMContextBuilder {
    // Convert account id to valid account id
    let predecessor_account_id: ValidAccountId = predecessor_account_id.try_into().unwrap();

    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id)
        .attached_deposit(deposit);
    builder
}

// Test that max storage sizes can accomodate reasonably large pawn offers / storage deposits
#[test]
pub fn test_max_storage_constants() {
    let pawn = Pawn {
        owner_id: String::from("owner_with_a_really_really_long_name"),
        nft_contract_id:  String::from("contract_id_with_a_really_really_long_name"),
        token_id: String::from("token_id_with_a_really_really_long_name"),
        loan_conditions: default_loan()
    };

    let pawn_size = pawn.try_to_vec().unwrap().len() as u128;
    let overhead = String::from("account_id_with_a_really_really_long_name")
        .try_to_vec().unwrap().len() as u128;

    assert!(pawn_size < super::MAX_STORAGE_PER_PAWN_OFFER as u128, "Max storage too small");
    assert!(overhead < super::MAX_OVERHEAD_PER_PAWN_OFFER as u128, "Max overhead too small");
}

#[test]
pub fn test_deposit_storage_successful() {
    let deposit = (super::MAX_OVERHEAD_PER_PAWN_OFFER as u128 + super::MAX_STORAGE_PER_PAWN_OFFER) 
        * env::storage_byte_cost() as u128;
    let context = get_context(bob(), deposit);
    
    testing_env!(context.build());

    let mut contract = Contract::new();
    contract.deposit_for_storage();

    assert!(contract.use_storage(&bob()), "Storage can't be used although paid for");
    assert!(!contract.use_storage(&bob()), "Storage used twice although paid for once");
}

#[test]
#[should_panic(expected="Insufficient deposit for storage")]
pub fn test_deposit_storage_insufficient_deposit() {
    let deposit = super::MAX_STORAGE_PER_PAWN_OFFER;
    let context = get_context(bob(), deposit);
    
    testing_env!(context.build());

    let mut contract = Contract::new();
    contract.deposit_for_storage();
}