use std::convert::TryFrom;

use near_sdk::{Promise, PromiseResult, Gas};

use crate::*;
use crate::external::*;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;

//TODO: Figure out this value
const GAS_FOR_TRANSFERRING_TOKEN: Gas = 80_000_000_000_000;
const GAS_FOR_RESOLVING_TRANSFER: Gas = 40_000_000_000_000;

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn repay_loan(&mut self, pawn_id: PawnId) {
        // Remove offered pawn. If not found, panic.
        let confirmed_pawn = self.confirmed_pawns.get(&pawn_id).expect("Pawn not found");

        assert_eq!(env::signer_account_id(), *confirmed_pawn.get_borrower(), "Signer not nft owner");
        
        let payment_amount = confirmed_pawn.get_payment_amount();
        assert!(env::attached_deposit() >= payment_amount.0, "Insufficient deposit to pay off loan");

        let broker_id = confirmed_pawn.get_broker().clone();
        let borrower_id = confirmed_pawn.get_borrower().clone();

        // Return the nft
        ext_nft::nft_transfer(
            validate(env::signer_account_id()), 
            confirmed_pawn.pawn.token_id, 
            Option::None, 
            Option::None, 
            &confirmed_pawn.pawn.nft_contract_id, 
            1, 
            GAS_FOR_TRANSFERRING_TOKEN
        )
        // Then, close the loan and the pay the broker
        .then(
            ext_self::resolve_return(
                pawn_id,
                payment_amount,
                borrower_id,
                broker_id,
                &env::current_account_id(),
                0,
                GAS_FOR_RESOLVING_TRANSFER
            )
        );
    }

    pub fn liquidate_pawn(&mut self, pawn_id: PawnId) {
        // Remove offered pawn. If not found, panic.
        let confirmed_pawn = self.confirmed_pawns.remove(&pawn_id).expect("Pawn not found");

        assert!(env::block_timestamp() > confirmed_pawn.get_deadline(), "Pawn redemption period not over");

        let broker_id = confirmed_pawn.get_broker().clone();
        let borrower_id = confirmed_pawn.get_borrower().clone();
        
        // Pawn can be liquidated, transfer NFT to broker
        ext_nft::nft_transfer(
            validate(env::signer_account_id()), 
            confirmed_pawn.pawn.token_id, 
            Option::None, 
            Option::None, 
            &confirmed_pawn.pawn.nft_contract_id, 
            1,
            GAS_FOR_TRANSFERRING_TOKEN
        )
        // Then, close the loan
        .then(
            ext_self::resolve_collect(
                pawn_id,
                borrower_id,
                broker_id,
                &env::current_account_id(),
                0,
                GAS_FOR_RESOLVING_TRANSFER
            )
        );
    }

    fn close_pawn(&mut self, pawn_id: PawnId, broker_id: AccountId, borrower_id: AccountId) {
        self.confirmed_pawns.remove(&pawn_id);
        
    }
}

trait NftTransferResolver {
    fn resolve_return(
        &mut self, 
        pawn_id: PawnId,
        payment_amount: LoanInYoctoNear,
        borrower_id: AccountId,
        broker_id: AccountId
    );

    fn resolve_collect(
        &mut self, 
        pawn_id: PawnId,
        borrower_id: AccountId,
        broker_id: AccountId
    );
}

#[near_bindgen]
impl NftTransferResolver for Contract {
    #[private]
    fn resolve_return(
        &mut self, 
        pawn_id: PawnId,
        payment_amount: LoanInYoctoNear,
        borrower_id: AccountId,
        broker_id: AccountId
    ) {
        // Ensure pawn has been properly returned to borrower
        assert_ne!(env::promise_result(0), PromiseResult::Failed, "Failed to return NFT to owner");

        // Transfer loan to owner
        Promise::new(broker_id.clone()).transfer(payment_amount.0);

        self.close_pawn(pawn_id, broker_id, borrower_id);
    }

    #[private]
    fn resolve_collect(
        &mut self, 
        pawn_id: PawnId,
        borrower_id: AccountId,
        broker_id: AccountId
    ) {
        // Ensure pawn has been properly returned to borrower
        assert_ne!(env::promise_result(0), PromiseResult::Failed, "Failed to transfer NFT to broker");

        self.close_pawn(pawn_id, broker_id, borrower_id);
    }
}