use near_sdk::{Gas, Balance, PromiseResult};

use crate::*;
use crate::external::{ext_nft, ext_self};

//TODO: Figure out this value
const GAS_FOR_TRANSFERRING_TOKEN: Gas = 80_000_000_000_000;
const GAS_FOR_RESOLVING_TRANSFER: Gas = 40_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum PendingTransfer {
    Outgoing { to: AccountId },
    Incoming { from: AccountId, approval_id: u64 } 
}

impl PendingTransfer {
    pub fn incoming(&self, error_message: &str) -> (AccountId, u64) {
        if let PendingTransfer::Incoming{ from, approval_id } = self {
            return (from.clone(), *approval_id);
        }

        assert!(false, "{}", error_message);
        ("".to_string(), 0)
    }

    pub fn outgoing(&self, error_message: &str) -> AccountId {
        if let PendingTransfer::Outgoing{ to } = self {
            return to.clone();
        }

        assert!(false, "{}", error_message);
        "".to_string()
    }
}

trait NftTransferResolver {
    fn list_pawn(
        &mut self, 
        owner_id: AccountId,
        nft_contract_id: AccountId, 
        token_id: TokenId, 
        loan_conditions: LoanConditions,
        deposit: Balance
    ) -> Pawn;

    fn resolve_transfer(
        &mut self, nft_contract_id: AccountId, token_id: TokenId
    );
}

#[near_bindgen]
impl NftTransferResolver for Contract {
    #[private]
    fn list_pawn(
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

    #[private]
    fn resolve_transfer(&mut self, nft_contract_id: AccountId, token_id: TokenId) {
        assert_ne!(env::promise_result(0), PromiseResult::Failed, "Failed to transfer NFT to pawnshop");

        let pawn_id = Pawn::pawn_id(&nft_contract_id, &token_id);
        self.pending_transfers.remove(&pawn_id);
    }
}

#[near_bindgen]
impl Contract {
    pub(crate) fn safe_transfer(&mut self, nft_contract_id: &AccountId, token_id: &TokenId, receiver_id: &AccountId) {

        let gas_left = env::prepaid_gas() - env::used_gas();
        assert!(
            gas_left > GAS_FOR_TRANSFERRING_TOKEN + GAS_FOR_RESOLVING_TRANSFER, 
            "Insufficient gas to safe transfer nft"
        );

        let pawn_id = Pawn::pawn_id(nft_contract_id, token_id);
        self.pending_transfers.insert(
            &pawn_id, 
            &PendingTransfer::Outgoing{ to: receiver_id.clone() }
        );

        // Initiate the transfer
        ext_nft::nft_transfer(
            validate(receiver_id.clone()),
            token_id.clone(), 
            Option::None, 
            Option::None, 
            nft_contract_id, 
            1,
            GAS_FOR_TRANSFERRING_TOKEN
        )
        // Then, close the pending transfer (if successful)
        .then(
            ext_self::resolve_transfer(
                nft_contract_id.clone(),
                token_id.clone(),
                &env::current_account_id(),
                0,
                GAS_FOR_RESOLVING_TRANSFER
            )
        );
    }

    /*
        A fallback function for users to call should safe_transfer() fail.
    */
    pub fn retry_outgoing_transfer(&mut self, nft_contract_id: AccountId, token_id: TokenId) {

        let pawn_id = Pawn::pawn_id(&nft_contract_id, &token_id);
        let receiver_id = self.pending_transfers.get(&pawn_id)
            .expect("No pending outgoing transfer")
            .outgoing("No pending outgoing transfer");

        assert!(
            env::prepaid_gas() > GAS_FOR_TRANSFERRING_TOKEN + GAS_FOR_RESOLVING_TRANSFER, 
            "Insufficient gas to safe transfer nft"
        );

        // Initiate the transfer
        ext_nft::nft_transfer(
            validate(receiver_id),
            token_id.clone(), 
            Option::None, 
            Option::None, 
            &nft_contract_id, 
            1,
            GAS_FOR_TRANSFERRING_TOKEN
        )
        // Then, close the pending transfer (if successful)
        .then(
            ext_self::resolve_transfer(
                nft_contract_id,
                token_id,
                &env::current_account_id(),
                0,
                GAS_FOR_RESOLVING_TRANSFER
            )
        );
    }
}