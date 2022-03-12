use near_sdk::ext_contract;
use near_sdk::json_types::ValidAccountId;

use crate::*;

/// external contract calls

#[ext_contract(ext_nft)]
trait NftCore {
    fn nft_transfer(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );

    fn nft_token(&self, token_id: TokenId) -> Option<Token>;
}

#[ext_contract(ext_self)]
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
        &mut self, 
        nft_contract_id: AccountId, 
        token_id: TokenId
    );
}