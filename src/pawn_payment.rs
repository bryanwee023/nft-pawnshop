use near_sdk::{Promise};

use crate::*;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;

#[near_bindgen]
impl Contract {

    /*
        Repay a previously granted loan. 
        Contract will transfer the repayment to the broker, and the NFT back to the borrower.
        Prerequisites:
            1. Deposit must cover loan value (with interest)
            2. Signer must be the borrower / nft owner
    */
    #[payable]
    pub fn repay_loan(&mut self, nft_contract_id: AccountId, token_id: TokenId) {
        let pawn_id = Pawn::pawn_id(&nft_contract_id, &token_id);

        // Remove offered pawn. If not found, panic.
        let confirmed_pawn = self.confirmed_pawns.get(&pawn_id).expect("Pawn not found");

        assert_eq!(env::signer_account_id(), *confirmed_pawn.get_borrower(), "Signer not nft owner");
        
        let payment_amount = confirmed_pawn.get_payment_amount();
        assert!(env::attached_deposit() >= payment_amount.0, "Insufficient deposit to pay off loan");

        let broker_id = confirmed_pawn.get_broker();
        let borrower_id = confirmed_pawn.get_borrower();

        // Remove pawn details from storage
        self.close_pawn(&pawn_id, broker_id, borrower_id);

        // Repay broker
        Promise::new(broker_id.clone()).transfer(payment_amount.0);

        // Return the nft
        self.safe_transfer(
            &nft_contract_id, 
            confirmed_pawn.get_token_id(), 
            confirmed_pawn.get_borrower()
        );
    }

    /*
        Collect the nft that was used to as collateral for an unpaid overdue ConfirmedPawn.
        Prerequisites:
            1. Pawn must be confirmed with an unreturned loan
            2. Pawn must be overdue
    */
    pub fn liquidate_pawn(&mut self, nft_contract_id: AccountId, token_id: TokenId) {
        let pawn_id = Pawn::pawn_id(&nft_contract_id, &token_id);

        // Remove offered pawn. If not found, panic.
        let confirmed_pawn = self.confirmed_pawns.remove(&pawn_id).expect("Pawn not found");

        assert!(env::block_timestamp() > confirmed_pawn.get_deadline(), "Pawn redemption period not over");

        let broker_id = confirmed_pawn.get_broker();
        let borrower_id = confirmed_pawn.get_borrower();

        // Remove pawn details from storage
        self.close_pawn(&pawn_id, broker_id, borrower_id);
        
        // Pawn can be liquidated, transfer NFT to broker
        self.safe_transfer(&nft_contract_id, &token_id, broker_id);
    }

    // Removes a given pawn (specified by id) from the contracts' persistent storage. 
    // Only use for resolved confirmed pawns
    fn close_pawn(&mut self, pawn_id: &PawnId, broker_id: &AccountId, borrower_id: &AccountId) {
        self.confirmed_pawns.remove(pawn_id);

        let borrower_pawns = self.by_borrower_id.get(borrower_id);
        if let Some(mut borrower_pawns) = borrower_pawns {
            borrower_pawns.remove(pawn_id);
            if borrower_pawns.len() > 0 {
                self.by_borrower_id.insert(borrower_id, &borrower_pawns);
            }
        }

        let broker_pawns = self.by_broker_id.get(broker_id);
        if let Some(mut broker_pawns) = broker_pawns {
            broker_pawns.remove(pawn_id);
            if broker_pawns.len() > 0 {
                self.by_borrower_id.insert(broker_id, &broker_pawns);
            }
        }

    }
}