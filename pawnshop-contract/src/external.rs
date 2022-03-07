use near_sdk::ext_contract;
use near_sdk::json_types::ValidAccountId;

use crate::*;

/// external contract calls

#[ext_contract(ext_nft)]
trait ExtContract {
    fn nft_transfer(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );
}