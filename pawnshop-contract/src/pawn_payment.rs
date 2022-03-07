use std::convert::TryFrom;

use near_sdk::{Promise, Gas};
use near_sdk::json_types::ValidAccountId;

use crate::*;
use crate::external::*;
//TODO: Figure out this value
const GAS_FOR_NFT_TRANSFER: Gas = 1000000;

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn repay_loan(&mut self, pawn_id: PawnId) {
        // Remove offered pawn. If not found, panic.
        let confirmed_pawn = self.pawns.remove(&pawn_id).expect("Pawn not found");

        assert_eq!(env::signer_account_id(), *confirmed_pawn.get_borrower(), "Signer not nft owner");
        
        let payment_amount = confirmed_pawn.get_payment_amount();
        assert!(env::attached_deposit() >= payment_amount.0);

        let borrower_id = ValidAccountId::try_from(env::signer_account_id()).expect("Invalid signer");

        // Repay loan and return nft
        Promise::new(confirmed_pawn.broker_id.to_string())
            .transfer(payment_amount.0)
            .then(
                ext_nft::nft_transfer(
                    borrower_id, 
                    confirmed_pawn.pawn.token_id, 
                    Option::None, 
                    Option::None, 
                    &confirmed_pawn.pawn.nft_contract_id, 
                    1, 
                    GAS_FOR_NFT_TRANSFER)
            );
    }

    pub fn liquidate_pawn(&mut self, pawn_id: PawnId) {
        // Remove offered pawn. If not found, panic.
        let confirmed_pawn = self.pawns.remove(&pawn_id).expect("Pawn not found");

        assert_eq!(env::signer_account_id(), confirmed_pawn.broker_id, "Signer not pawnbroker");
        assert!(env::block_timestamp() > confirmed_pawn.get_deadline(), "Pawn redemption period not over");

        let broker_id = ValidAccountId::try_from(env::signer_account_id()).expect("Invalid signer");
        
        // Pawn can be liquidated, transfer NFT
        ext_nft::nft_transfer(
            broker_id, 
            confirmed_pawn.pawn.token_id, 
            Option::None, 
            Option::None, 
            &confirmed_pawn.pawn.nft_contract_id, 
            1,
            GAS_FOR_NFT_TRANSFER
        );
    }
}