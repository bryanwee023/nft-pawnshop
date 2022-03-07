use near_sdk::PromiseOrValue;
use near_contract_standards::non_fungible_token::core::NonFungibleTokenReceiver;

use crate::*;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;

#[near_bindgen]
impl NonFungibleTokenReceiver for Contract {

    // On transfers, contract will verify the transfer and subsequently list nft for pawning
    // As with NEP-171 standard, callback returns false if transfer is accepted. Returns true otherwise.
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool> { 
        let loan_conditions = near_sdk::serde_json::from_str::<LoanConditions>(&msg).expect("Invalid Loan Conditions}");

        let res = self.offer_pawn(
            previous_owner_id,
            sender_id, 
            token_id, 
            loan_conditions
        );

        return PromiseOrValue::Value(!res);
    }

}