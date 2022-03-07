use std::convert::TryInto;

use super::*;
use crate::test_utils::*;

use near_sdk::json_types::ValidAccountId;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::MockedBlockchain;
use near_sdk::testing_env;

const VALID_PAWN_MESSAGE: &str = r#"
    {
        "loan_value": "156543",
        "interest": 15,
        "duration": 15211
    }
"#;

fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id)
        .block_timestamp(start_time());
    builder
}


#[test]
fn test_on_transfer_successful() {
    let context = get_context(bob().try_into().unwrap());
    testing_env!(context.build());

    let mut contract = Contract::new();
    contract.nft_on_transfer(
        nft_contract().to_string(), 
        alice().to_string(), 
        token_id(), 
        VALID_PAWN_MESSAGE.to_string()
    );
}

#[test]
#[should_panic(expected = "Invalid Loan Conditions")]
fn test_invalid_msg_failure() {
    let context = get_context(bob().try_into().unwrap());
    testing_env!(context.build());

    let mut contract = Contract::new();
    contract.nft_on_transfer(
        nft_contract().to_string(), 
        alice().to_string(), 
        token_id(), 
        "An Invalid Message".to_string()
    );
}
