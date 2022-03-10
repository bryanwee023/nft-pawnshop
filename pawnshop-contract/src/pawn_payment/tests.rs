use super::*;
use crate::test_utils::*;

use near_sdk::json_types::ValidAccountId;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{MockedBlockchain, Balance};
use near_sdk::testing_env;

fn get_context(predecessor_account_id: AccountId, deposit: Balance, timestamp: Time) -> VMContextBuilder {
    let predecessor_account_id: ValidAccountId = predecessor_account_id.try_into().unwrap();

    let mut context = VMContextBuilder::new();
    context
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id)
        .attached_deposit(deposit)
        .block_timestamp(timestamp);

    context
}

fn get_contract_with_confirmed_pawn() -> Contract {
    let mut contract = Contract::new();

    contract.confirmed_pawns.insert(
        &pawn_id(),
        &default_confirmed_pawn()
    );
    contract
}

#[test]
fn test_repay_pawn_successful() {
    let context = get_context(
        alice(), 
        default_confirmed_pawn().get_payment_amount().0,
        start_time()
    ).build();
    testing_env!(context);

    let mut contract = get_contract_with_confirmed_pawn();

    contract.repay_loan(pawn_id());

    assert!(contract.confirmed_pawn(pawn_id()).is_none());
}

#[test]
#[should_panic(expected = "Insufficient deposit to pay off loan")]
fn test_repay_pawn_insufficient_deposit_fail() {
    let context = get_context(
        alice(), 
        default_confirmed_pawn().get_payment_amount().0 - 100_000,
        start_time()
    ).build();
    testing_env!(context);

    let mut contract = get_contract_with_confirmed_pawn();

    contract.repay_loan(pawn_id());
}

#[test]
fn test_liquidate_pawn_successful() {
    let context = get_context(
        alice(), 
        default_confirmed_pawn().get_payment_amount().0,
        start_time() + default_loan().duration + 100000 // Right after due date
    ).build();
    testing_env!(context);

    let mut contract = get_contract_with_confirmed_pawn();

    contract.liquidate_pawn(pawn_id());

    assert!(contract.confirmed_pawn(pawn_id()).is_none());
}

#[test]
#[should_panic(expected="Pawn redemption period not over")]
fn test_liquidate_pawn_before_due_date_fail() {
    let context = get_context(
        alice(), 
        default_confirmed_pawn().get_payment_amount().0,
        start_time() + default_loan().duration - 100000 // Right before due date
    ).build();
    testing_env!(context);

    let mut contract = get_contract_with_confirmed_pawn();

    contract.liquidate_pawn(pawn_id());

    assert!(contract.confirmed_pawn(pawn_id()).is_none());
}