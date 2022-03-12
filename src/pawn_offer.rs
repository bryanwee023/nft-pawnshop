use near_sdk::{Promise, Gas, PromiseOrValue};

use crate::*;
use crate::external::*;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;

//TODO: Figure out these values
const TOTAL_GAS_FOR_OFFER: Gas = 125_000_000_000_000;
const GAS_FOR_TRANSFERRING_TOKEN: Gas = 80_000_000_000_000;
const GAS_FOR_LISTING_PAWN: Gas = 40_000_000_000_000;
const TOTAL_GAS_FOR_WITHDRAWAL: Gas = 100_000_000_000_000;

#[near_bindgen]
impl Contract {
    // Processes a pawn offer by the nft owner. 
    #[payable]
    pub fn offer_pawn(
        &mut self,
        nft_contract_id: AccountId, 
        token_id: TokenId, 
        loan_conditions: LoanConditions
    ) -> PromiseOrValue<Pawn> {
        // Check for gas early
        assert!(env::prepaid_gas() >= TOTAL_GAS_FOR_OFFER, "Insufficient gas");

        // TODO: Check for storage deposit early

        let pawn_id = Pawn::pawn_id(&nft_contract_id, &token_id);
        let (owner_id, approval_id) = self.pending_transfers.get(&pawn_id)
            .expect("NFT not approved")
            .incoming("NFT not approved");

        assert_eq!(owner_id, env::signer_account_id(), "NFT does not belong to signer");

        // Initiate cross-contract call
        ext_nft::nft_transfer(
            validate(env::current_account_id()), 
            token_id.clone(), 
            Option::Some(approval_id), 
            Option::None,
            &nft_contract_id,
            1,
            GAS_FOR_TRANSFERRING_TOKEN
        )
        .then(ext_self::list_pawn(
            env::signer_account_id(),
            nft_contract_id, 
            token_id, 
            loan_conditions,
            env::attached_deposit(),
            &env::current_account_id(),
            0,
            GAS_FOR_LISTING_PAWN
        ))
        .into()
    }

    /*
        Withdraw a pawn offer.
    */
    pub fn withdraw_offer(
        &mut self,
        nft_contract_id: AccountId, 
        token_id: TokenId, 
    ) {
        assert!(env::prepaid_gas() >= TOTAL_GAS_FOR_WITHDRAWAL, "Insufficient gas");

        let pawn_id = Pawn::pawn_id(&nft_contract_id, &token_id);

        // Attempt to remove offered pawn.
        let pawn = self.offered_pawns.remove(&pawn_id).expect("Pawn offer not found");

        assert_eq!(pawn.owner_id, env::signer_account_id(), "Only nft owner can revoke offer");
        self.by_borrower_id.remove(&pawn_id); 
    
        self.safe_transfer(&nft_contract_id, &token_id, &pawn.owner_id);
    }

    #[payable]
    pub fn accept_pawn(&mut self, pawn_id: PawnId) -> ConfirmedPawn {
        // Remove offered pawn. If not found, panic.
        let pawn = self.offered_pawns.remove(&pawn_id).expect("Pawn not found");

        assert!(env::attached_deposit() >= pawn.get_loan_value().0, "Insufficient deposit to facilitate loan");
        assert_ne!(env::signer_account_id(), pawn.owner_id, "broker_id should be not owner_id");

        let initial_storage = env::storage_usage();
        let broker_id = env::signer_account_id();
        let borrower_id = pawn.owner_id.clone();
        let loan_value = pawn.get_loan_value().0;

        // Move pawn into map of confirmed pawns
        let confirmed_pawn = ConfirmedPawn {
            pawn,
            broker_id: broker_id.clone(),
            start_time: env::block_timestamp()
        };

        self.confirmed_pawns.insert(
            &pawn_id, 
            &confirmed_pawn
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

        // Transfer loan
        Promise::new(borrower_id).transfer(loan_value);

        //confirmed_pawn
        confirmed_pawn
    }

}