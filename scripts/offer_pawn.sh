#!/usr/bin/env bash
cd "`dirname $0`"
. ./config.sh

near call $PAWNSHOP_ID offer_pawn '{
    "nft_contract_id": "'$NFT_CONTRACT_ID'", 
    "token_id": "'$TOKEN_ID'", 
    "loan_conditions": {
        "loan_value":"'$LOAN_VALUE_YOCTO'", 
        "interest":'$INTEREST', 
        "duration":'$DURATION' 
    }
}' --deposit 0.01 --accountId $BORROWER_ID --gas=$MORE_GAS