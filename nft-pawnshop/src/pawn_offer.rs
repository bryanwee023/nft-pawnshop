use near_sdk::{Promise, Gas, Balance, PromiseOrValue, PromiseResult};

use crate::*;
use crate::external::{ext_nft, ext_self};

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;

//TODO: Figure out these values
const GAS_FOR_INITIATING_OFFER: Gas = 3_000_000_000_000;
const GAS_FOR_TRANSFERRING_TOKEN: Gas = 80_000_000_000_000;
const GAS_FOR_RESOLVING_TRANSFER: Gas = 40_000_000_000_000;
const GAS_FOR_INITIATING_WITHDRAWAL: Gas = 10_000_000_000_000;

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
        let estimated_gas = GAS_FOR_INITIATING_OFFER + GAS_FOR_TRANSFERRING_TOKEN + GAS_FOR_RESOLVING_TRANSFER;
        assert!(env::prepaid_gas() >= estimated_gas, "Insufficient gas");

        // TODO: Check for storage deposit early

        let pawn_id = Pawn::pawn_id(&nft_contract_id, &token_id);
        let (owner_id, approval_id) = self.pending_transfers.get(&pawn_id).expect("NFT not approved");

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
        .then(ext_self::resolve_transfer(
            owner_id,
            nft_contract_id, 
            token_id, 
            loan_conditions,
            env::attached_deposit(),
            &env::current_account_id(),
            0,
            GAS_FOR_RESOLVING_TRANSFER
        ))
        .into()
    }
    /*
        Withdraw a pawn offer. Pawn must not have been confirmed.\
        Two possible cases: 
            1. The user wishes to withdraw a processed but unconfirmed offer
            2. The user wishes to withdraw a transferred but unprocessed offer
        The latter is possible if the callback resolve_transfer() panicked
    */
    pub fn withdraw_offer(
        &mut self,
        nft_contract_id: AccountId, 
        token_id: TokenId, 
    ) {
        let pawn_id = Pawn::pawn_id(&nft_contract_id, &token_id);

        // Attempt to remove offered pawn.
        let pawn = self.offered_pawns.remove(&pawn_id);

        let receiver_id = match pawn {
            // Case 1: Withdrawing a processed but unconfirmed nft
            Option::Some(x) => {
                assert_eq!(x.owner_id, env::signer_account_id(), "Only nft owner can revoke offer");
                self.by_borrower_id.remove(&pawn_id); 
                x.owner_id
            },
            // Case 2: Withdrawing a transferred but unprocessed nft
            Option::None => {
                assert!(self.confirmed_pawns.get(&pawn_id).is_none(), "Cannot revoke an already pawned item");
                self.pending_transfers.get(&pawn_id).unwrap().0 
                // TODO: Remove pawn_id from pending transfers once transfer confirmed.
                // But what if nft_transfer fails?
            }
        };

        // Safe to return nft to owner
        ext_nft::nft_transfer(
            validate(receiver_id), 
            token_id, 
            Option::None, 
            Option::None, 
            &nft_contract_id, 
            1, 
            env::prepaid_gas() - GAS_FOR_INITIATING_WITHDRAWAL
        );
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

trait NftTransferResolver {
    fn resolve_transfer(
        &mut self, 
        owner_id: AccountId,
        nft_contract_id: AccountId, 
        token_id: TokenId, 
        loan_conditions: LoanConditions,
        deposit: Balance
    ) -> Pawn;
}

#[near_bindgen]
impl NftTransferResolver for Contract {
    #[private]
    fn resolve_transfer(
        &mut self, 
        owner_id: AccountId,
        nft_contract_id: AccountId, 
        token_id: TokenId, 
        loan_conditions: LoanConditions,
        deposit: Balance
    ) -> Pawn {
        assert_ne!(env::promise_result(0), PromiseResult::Failed, "Failed to transfer NFT to pawnshop");

        let initial_storage = env::storage_usage();

        let pawn = Pawn {
            owner_id: owner_id.clone(),
            nft_contract_id,
            token_id,
            loan_conditions
        };
        let pawn_id = pawn.get_pawn_id();

        self.pending_transfers.remove(&pawn_id);

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

        // Check that initial deposit can cover storage
        let storage_used = (env::storage_usage() - initial_storage) as Balance;
        assert!(deposit >= storage_used * env::STORAGE_PRICE_PER_BYTE, "Initial deposit insufficient to pay for storage");

        pawn
    }
}