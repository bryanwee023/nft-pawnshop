use near_sdk::json_types::U128;

use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct LoanConditions {
    pub loan_value: LoanInYoctoNear,
    pub interest: InterestInPercent,
    pub duration: Time
}

//struct that holds important information about each sale on the market
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Pawn {
    pub owner_id: AccountId,
    pub nft_contract_id: AccountId,
    pub token_id: String,
    pub loan_conditions: LoanConditions
}

impl Pawn {
    pub fn get_loan_value(&self) -> &LoanInYoctoNear { &self.loan_conditions.loan_value }
    pub fn get_interest(&self) -> &InterestInPercent { &self.loan_conditions.interest }
    pub fn get_duration(&self) -> &Time { &self.loan_conditions.duration }

    pub fn get_pawn_id(&self) -> PawnId {
        format!("{}.{}", self.nft_contract_id, self.token_id)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct ConfirmedPawn {
    pub pawn: Pawn,
    pub broker_id: AccountId,
    pub start_time: Time
}

impl ConfirmedPawn {
    pub fn get_borrower(&self) -> &AccountId { &self.pawn.owner_id }
    pub fn get_nft_id(&self) -> &AccountId { &self.pawn.nft_contract_id }
    pub fn get_token_id(&self) -> &TokenId { &self.pawn.token_id }
    pub fn get_pawn_id(&self) -> PawnId { self.pawn.get_pawn_id() }

    pub fn get_loan_value(&self) -> &LoanInYoctoNear { self.pawn.get_loan_value() }
    pub fn get_interest(&self) -> &InterestInPercent { self.pawn.get_interest() }
    pub fn get_duration(&self) -> &Time { self.pawn.get_duration() }

    pub fn get_deadline(&self) -> Time {
        self.start_time + self.get_duration()
    }

    pub fn get_payment_amount(&self) -> LoanInYoctoNear {
        U128(self.get_loan_value().0 * (100 + self.get_interest()) as u128 / 100)
    }
}

