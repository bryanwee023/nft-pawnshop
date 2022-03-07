use crate::*;

const MAX_STORAGE_PER_PAWN_OFFER: u128 = 500; // Maximum number of bytes adding a new pawn offer should need
const MAX_OVERHEAD_PER_PAWN_OFFER: u64 = 100; // Maximum number of bytes adding a new storage deposit should need

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn deposit_for_storage(&mut self) {
        let initial_storage = env::storage_usage();

        let required_deposit = MAX_STORAGE_PER_PAWN_OFFER * env::storage_byte_cost();

        assert!(env::attached_deposit() >= required_deposit, "Insufficient deposit for storage");

        let account_id = env::signer_account_id();
        assert!(
            !self.storage_deposits.contains(&account_id),
            "Account already paid for deposit"
        );

        self.storage_deposits.insert(&account_id);

        let storage_increase = env::storage_usage() - initial_storage;
        assert!(storage_increase < MAX_OVERHEAD_PER_PAWN_OFFER
            , "Inserting new deposit requires {} bytes", storage_increase);
    }

    pub(crate) fn use_storage(&mut self, account_id: &AccountId) -> bool {
        self.storage_deposits.remove(account_id)
    }
}