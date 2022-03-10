/*
    Mainly focused on testing brokers accepting pawns.
    Tests for offering pawn can be found in simulation tests.
*/
use super::*;
use crate::test_utils::*;

use near_sdk::json_types::ValidAccountId;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{MockedBlockchain, Balance};
use near_sdk::testing_env;

const DEPOSIT_FOR_STORAGE: u128 = 5_000_000_000_000_000_000_000; //0.005 Near

fn get_context(predecessor_account_id: AccountId, deposit: Balance) -> VMContextBuilder {
    let predecessor_account_id: ValidAccountId = predecessor_account_id.try_into().unwrap();

    let mut context = VMContextBuilder::new();
    context
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id)
        .attached_deposit(deposit)
        .block_timestamp(start_time());

    context
}

fn get_contract_with_offer() -> Contract {
    let mut contract = Contract::new();

    contract.offered_pawns.insert(
        &pawn_id(),
        &default_pawn()
    );
    contract
}

#[test]
fn test_accept_pawn_successful() {
    let context = get_context(
            bob(), 
            default_loan().loan_value.0 + DEPOSIT_FOR_STORAGE
        ).build();
    testing_env!(context);

    let mut contract = get_contract_with_offer();

    let confirmed_pawn = contract.accept_pawn(pawn_id());

    assert!(default_confirmed_pawn() == confirmed_pawn);
    assert!(contract.confirmed_pawn(pawn_id()).unwrap() == confirmed_pawn);
}

#[test]
#[should_panic(expected = "Insufficient deposit to facilitate loan and storage")]
fn test_accept_pawn_unpaid_storage_fail() {
    let context = get_context(
            bob(), 
            default_loan().loan_value.0
        ).build();
    testing_env!(context);

    let mut contract = get_contract_with_offer();

    let _ = contract.accept_pawn(pawn_id());
}

#[test]
#[should_panic(expected = "Pawn not found")]
fn test_accept_inexistent_pawn_fail() {
    let context = get_context(
            bob(), 
            default_loan().loan_value.0 + DEPOSIT_FOR_STORAGE
        ).build();
    testing_env!(context);

    let mut contract = Contract::new();

    let _ = contract.accept_pawn(pawn_id());
}

#[test]
#[should_panic(expected = "Pawn not found")]
fn test_accept_pawn_twice_fail() {
    let context = get_context(
            bob(), 
            default_loan().loan_value.0 + DEPOSIT_FOR_STORAGE
        ).build();
    testing_env!(context);

    let mut contract = get_contract_with_offer();

    let _ = contract.accept_pawn(pawn_id());
    let _ = contract.accept_pawn(pawn_id());
}