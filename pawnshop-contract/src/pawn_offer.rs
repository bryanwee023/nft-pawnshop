use near_sdk::Promise;

use crate::*;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;

#[near_bindgen]
impl Contract {
    // Processes a pawn offer by the nft owner. Should only be called by nft_on_transfer() [See nft_]
    // Returns true if successful. False otherwise.
    #[private]
    pub(crate) fn offer_pawn(
        &mut self, owner_id: AccountId, 
        nft_contract_id: AccountId, 
        token_id: TokenId, 
        loan_conditions: LoanConditions
    ) -> bool {
        // Ensure borrower has paid for storage
        if !self.use_storage(&owner_id) {
            return false;
        }

        let pawn = Pawn {
            owner_id: owner_id.clone(),
            nft_contract_id,
            token_id,
            loan_conditions
        };
        let pawn_id = pawn.get_pawn_id();

        // Add pawn to list of offered pawns
        self.offered_pawns.insert(
            &pawn.get_pawn_id(),
            &pawn
        );

        // Update borrower's set of pawned tokens
        let mut pawned_tokens = self.by_borrower_id.get(&owner_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByBorrowerIdInner {account_id_hash: hash_account_id(&owner_id)}
            )
        });

        pawned_tokens.insert(&pawn_id);
        self.by_borrower_id.insert(&owner_id, &pawned_tokens);

        true
    }

    #[payable]
    pub fn accept_pawn(&mut self, pawn_id: PawnId) {
        // Remove offered pawn. If not found, panic.
        let pawn = self.offered_pawns.remove(&pawn_id).expect("Pawn not found");

        assert!(env::attached_deposit() >= pawn.get_loan_value().0, "Insufficient deposit to facilitate loan");
        assert_ne!(env::signer_account_id(), pawn.owner_id, "broker_id should be not owner_id");

        let initial_storage = env::storage_usage();
        let broker_id = env::signer_account_id();
        let borrower_id = pawn.owner_id.clone();
        let loan_value = pawn.get_loan_value().0;

        // Move pawn into map of confirmed pawns
        self.pawns.insert(
            &pawn_id, 
            &ConfirmedPawn {
                pawn,
                broker_id: broker_id.clone(),
                start_time: env::block_timestamp()
            }
        );

        // Update broker's set of pawned tokens
        let mut pawned_tokens = self.by_broker_id.get(&broker_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByBrokerIdInner {account_id_hash: hash_account_id(&broker_id)}
            )
        });

        pawned_tokens.insert(&pawn_id);
        self.by_borrower_id.insert(&broker_id, &pawned_tokens);

        // Transfer (loan - storage cost) to borrower
        let storage_cost = (env::storage_usage() - initial_storage) as u128 * env::storage_byte_cost();

        assert!(env::attached_deposit() >= loan_value + storage_cost, "Insufficient deposit to facilitate loan and storage");

        Promise::new(borrower_id)
            .transfer(env::attached_deposit() - storage_cost);
    }

}
