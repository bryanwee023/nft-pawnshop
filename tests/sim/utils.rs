use near_sdk_sim::{ContractAccount, deploy, UserAccount};

use nft_pawnshop::ContractContract as PawnshopContract;
use non_fungible_token::ContractContract as NftContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    PAWNSHOP_BYTES => "out/nft_pawnshop.wasm",
    NFT_BYTES => "out/non_fungible_token.wasm"
}

const PAWNSHOP_ID: &str = "nft_pawnshop";
const NFT_ID: &str = "nft_contract";

pub(crate) fn get_contracts(root: UserAccount) -> (ContractAccount<PawnshopContract>, ContractAccount<NftContract>) {
    let pawnshop: ContractAccount<PawnshopContract> = deploy!(
        contract: PawnshopContract,
        contract_id: PAWNSHOP_ID.to_string(),
        bytes: &PAWNSHOP_BYTES,
        signer_account: root,
        init_method: new()
    );

    let nft: ContractAccount<NftContract> = deploy!(
        contract: NftContract,
        contract_id: NFT_ID.to_string(),
        bytes: &NFT_BYTES,
        signer_account: root,
        init_method: new_default_meta(
            root.account_id().try_into().unwrap()
        )
    );

    (pawnshop, nft)
}