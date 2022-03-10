use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::AccountId;
use near_sdk::{env, near_bindgen, PanicOnDefault};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};

use crate::pawn::*;
use crate::utils::*;

mod pawn;
mod pawn_offer;
mod pawn_payment;
mod approval_receiver;
mod external;
mod utils;

#[cfg(test)]
mod test_utils;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub confirmed_pawns: LookupMap<PawnId, ConfirmedPawn>,
    pub offered_pawns: LookupMap<PawnId, Pawn>,
    pub by_broker_id: LookupMap<AccountId, UnorderedSet<PawnId>>,
    pub by_borrower_id: LookupMap<AccountId, UnorderedSet<PawnId>>,
    pub pending_transfers: LookupMap<AccountId, (PawnId, u64)>
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new() -> Self {
        let this = Self {
            confirmed_pawns: LookupMap::new(StorageKey::Pawns),
            offered_pawns: LookupMap::new(StorageKey::OfferedPawns),
            by_broker_id: LookupMap::new(StorageKey::ByBrokerId),
            by_borrower_id: LookupMap::new(StorageKey::ByBorrowerId),
            pending_transfers: LookupMap::new(StorageKey::ByUserId)
        };

        this
    }

    pub fn pending_transfer(&self, pawn_id: PawnId) -> Option<(PawnId, u64)>  {
        self.pending_transfers.get(&pawn_id)
    }

    pub fn pawns_by_broker(&self, account_id: AccountId) -> Vec<PawnId>  {
        let pawn_ids = if let Some(pawn_ids) = self.by_broker_id.get(&account_id) {
            pawn_ids
        } else {
            return vec![];
        };

        pawn_ids
            .iter()
            .collect()
    }

    pub fn pawns_by_borrower(&self, account_id: AccountId) -> Vec<PawnId>  {
        let pawn_ids = if let Some(pawn_ids) = self.by_broker_id.get(&account_id) {
            pawn_ids
        } else {
            return vec![];
        };

        pawn_ids
            .iter()
            .collect()
    }

    pub fn offered_pawn(&self, pawn_id: PawnId) -> Option<Pawn>  {
        self.offered_pawns.get(&pawn_id)
    }

    pub fn confirmed_pawn(&self, pawn_id: PawnId) -> Option<ConfirmedPawn>  {
        self.confirmed_pawns.get(&pawn_id)
    }
}

// Unit Tests
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::testing_env;

    use super::*;
    
    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        testing_env!(context.is_view(true).build());
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }
}