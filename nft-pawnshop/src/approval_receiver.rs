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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::MockedBlockchain;
    use near_sdk::testing_env;
    use near_contract_standards::non_fungible_token::approval::NonFungibleTokenApprovalReceiver;

    use crate::Contract;
    use crate::test_utils::*;
    use super::*;

    #[test]
    fn on_approve_success() {
        let context = VMContextBuilder::new()
            .current_account_id(validate(pawnshop_id()))
            .signer_account_id(validate(alice()))
            .predecessor_account_id(validate(nft_contract()))
            .build();
        testing_env!(context);

        let mut contract = Contract::new();
        contract.nft_on_approve(token_id(), alice(), 11, String::new());

        let pawn_id = Pawn::pawn_id(&nft_contract(), &token_id());
        let expected_pending = contract.pending_transfer(pawn_id).unwrap();
        assert_eq!(expected_pending, (alice(), 11));
    }
}