#!/usr/bin/env bash
cd "`dirname $0`"
. ./config.sh

near call $PAWNSHOP_ID liquidate_pawn '{     
    "nft_contract_id": "'$NFT_CONTRACT_ID'", 
    "token_id": "'$TOKEN_ID'"
}' --accountId $BROKER_ID --gas=$MORE_GAS