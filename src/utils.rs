use std::convert::TryInto;
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::json_types::{U128, ValidAccountId};
use near_sdk::AccountId;
use near_sdk::{env, CryptoHash, BorshStorageKey};

pub type TokenId = String;
pub type LoanInYoctoNear = U128;
pub type InterestInPercent = u16;
pub type PawnId = String;
pub type Time = u64;

// Converts AccountId into ValidAccountId
pub(crate) fn validate(account_id: AccountId) -> ValidAccountId {
    account_id.try_into().unwrap()
}

// Used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.to_string().as_bytes()));
    hash
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Pawns,
    OfferedPawns,
    ByBrokerId,
    ByBrokerIdInner { account_id_hash: CryptoHash },
    ByBorrowerId,
    ByBorrowerIdInner { account_id_hash: CryptoHash },
    ByUserId
}