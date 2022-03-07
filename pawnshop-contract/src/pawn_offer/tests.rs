use std::convert::TryInto;

use super::*;
use crate::test_utils::*;

use near_sdk::env::storage_byte_cost;
use near_sdk::json_types::ValidAccountId;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{MockedBlockchain, Balance};
use near_sdk::testing_env;

// Helper Functions
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

fn get_contract(has_storage_deposit: bool) -> Contract {

    let mut contract = Contract::new();

    if has_storage_deposit { 
        let storage_deposit = 300 * env::STORAGE_PRICE_PER_BYTE;
        let context = get_context(alice(), storage_deposit);
        testing_env!(context.build());

        contract.deposit_for_storage(); 
    }

    contract
}

fn get_contract_with_offer() -> Contract {
    let mut contract = get_contract(true);

    contract.offer_pawn(alice(), nft_contract(), token_id(), default_loan());

    contract
}


// Tests 
#[test]
fn test_offer_pawn_successful() {
    let mut contract = get_contract(true);

    contract.offer_pawn(
    alice().to_string(), nft_contract(), 
    token_id(), default_loan()
    );

    // Test whether pawn is listed
    assert!(
        contract.offered_pawns.get(&pawn_id())
            .map(|x| x == default_pawn())
            .unwrap_or(false),
        "Offered pawn not found"
    );
    // Test whether pawn is mapped to the owner
    assert!(
        contract.by_borrower_id.get(&alice())
            .map(|x| x.contains(&pawn_id()))
            .unwrap_or(false),
        "Offered pawn not mapped to borrower"
    );
}

#[test]
fn test_offer_pawn_unpaid_storage_fail() {
    let mut contract = get_contract(false);

    let context = get_context(alice(), 0);
    testing_env!(context.build());
    
    assert!(
        !contract.offer_pawn(
            alice(), nft_contract(), 
            token_id(), default_loan()
        )
    );
}

// TODO: Move to Simulation tests, need to test if borrower received loan.
#[test]
fn test_accept_pawn_successful() {
    let mut contract = get_contract_with_offer();

    let storage_cost = 500 * storage_byte_cost();

    let context = get_context(bob(), loan_value().0 + storage_cost);
    testing_env!(context.build());

    contract.accept_pawn(pawn_id());

    // Ensure pawn is no longer listed
    assert!(
        contract.offered_pawns.get(&pawn_id())
            .map(|x| x == default_pawn())
            .is_none(),
        "Pawn still being listed"
    );

    contract.pawns.get(&pawn_id())
        .map(|x| println!("{}\n", x.broker_id));
    

    // Test whether pawn is confirmed
    assert!(
        contract.pawns.get(&pawn_id())
            .map(|x| x == default_confirmed_pawn())
            .unwrap_or(false),
        "Confirmed pawn not found"
    );

    // Test whether pawn is mapped to the owner
    assert!(
        contract.by_borrower_id.get(&bob())
            .map(|x| x.contains(&pawn_id()))
            .unwrap_or(false),
        "Confirmed pawn not mapped to broker"
    );
}

#[test]
#[should_panic(expected = "Pawn not found")]
fn test_accept_inexistent_pawn_fail() {
    let mut contract = get_contract(true);

    let context = get_context(bob(), loan_value().0);
    testing_env!(context.build());

    contract.accept_pawn(String::from("invalid_id"));
}

#[test]
#[should_panic(expected = "Insufficient deposit to facilitate loan")]
fn test_accept_pawn_insufficient_deposit_fail() {
    let mut contract = get_contract_with_offer();

    let context = get_context(bob(), loan_value().0 / 2);
    testing_env!(context.build());

    contract.accept_pawn(pawn_id());
}

#[test]
#[should_panic(expected = "Insufficient deposit to facilitate loan and storage")]
fn test_accept_pawn_unpaid_storage_fail() {
    let mut contract = get_contract_with_offer();

    let context = get_context(bob(), loan_value().0);
    testing_env!(context.build());

    contract.accept_pawn(pawn_id());
}
