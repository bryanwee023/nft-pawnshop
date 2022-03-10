#!/usr/bin/env bash
. ./nft-scripts/config.sh

near call $NFT_CONTRACT_ID nft_transfer_call "{
        \"receiver_id\": \"$RECEIVER_ID\", 
        \"token_id\": \"$TOKEN_ID\", 
        \"approval_id\": 0,
        \"msg\": \"$LOAN_CONDITIONS\"
    }" --accountId $NFT_OWNER_ID --depositYocto 1 --gas $GAS_FOR_TRANSFER_CALL