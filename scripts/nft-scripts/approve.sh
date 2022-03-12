#!/usr/bin/env bash
cd "`dirname $0`"
. ./config.sh

near call $NFT_CONTRACT_ID nft_approve '{
    "token_id": "'$TOKEN_ID'", 
    "account_id": "'$APPROVED_CONTRACT_ID'", 
    "msg": "As collateral"
}' --accountId $NFT_OWNER_ID --deposit 0.1