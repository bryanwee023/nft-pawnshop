/*
Issue: Malicious user can lock contract's balance by occupying storage without payment
Reason: By convention, NFT contracts do not deposit a balance when calling nft_on_approve()

Possible Fix: Allow administrator to revoke approvals
*/
use near_contract_standards::non_fungible_token::approval::NonFungibleTokenApprovalReceiver;
use near_sdk::PromiseOrValue;

use crate::*;

#[near_bindgen]
impl NonFungibleTokenApprovalReceiver for Contract {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) -> PromiseOrValue<String> {
        let pawn_id = Pawn::pawn_id(&env::predecessor_account_id(), &token_id);
        self.pending_transfers.insert(&pawn_id, &(owner_id, approval_id));

        PromiseOrValue::Value(String::from("Transfer request submitted"))
    }
}