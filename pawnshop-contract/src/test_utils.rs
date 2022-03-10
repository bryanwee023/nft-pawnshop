use near_sdk::AccountId;
use near_sdk::json_types::{U128, ValidAccountId};
use near_sdk::test_utils::accounts;

use crate::pawn::{Pawn, ConfirmedPawn, LoanConditions};
use crate::utils::*;

pub(crate) fn validate(account_id: AccountId) -> ValidAccountId {
    account_id.try_into().unwrap()
}

pub(crate) fn alice() -> AccountId { accounts(0).to_string() }
pub(crate) fn bob() -> AccountId { accounts(1).to_string() }
pub(crate) fn pawnshop_id() -> AccountId { "my_pawnshop".to_string() }

pub(crate) fn nft_contract() -> AccountId { accounts(1).to_string() }
pub(crate) fn token_id() -> TokenId { String::from("my token") }
pub(crate)fn pawn_id() -> PawnId { 
    format!("{}.{}", nft_contract(), token_id())
}

pub (crate) fn default_loan() -> LoanConditions {
    LoanConditions {
        loan_value: U128(10_000_000_000_000_000_000_000_000),
        interest: 20,
        duration:  123456787654321
    }
}
pub(crate) fn start_time() -> Time { 12345678 }

pub(crate) fn default_pawn() -> Pawn {
    Pawn {
        owner_id: alice().to_string(),
        nft_contract_id: nft_contract().to_string(),
        token_id: token_id(),
        loan_conditions: default_loan()
    }
}

pub(crate) fn default_confirmed_pawn() -> ConfirmedPawn {
    ConfirmedPawn {
        pawn: default_pawn(),
        broker_id: bob().to_string(),
        start_time: start_time()
    }
}