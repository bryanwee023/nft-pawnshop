#!/usr/bin/env bash
cd "`dirname $0`"
. ./config.sh

near call $NFT_CONTRACT_ID nft_mint '{
    "token_id": "'$TOKEN_ID'", 
    "receiver_id": "'$NFT_OWNER_ID'", 
    "token_metadata": { 
        "title": "'$TOKEN_ID'", 
        "description": "For testing purposes", 
        "media": "https://bafybeidl4hjbpdr6u6xvlrizwxbrfcyqurzvcnn5xoilmcqbxfbdwrmp5m.ipfs.dweb.link/", 
        "copies": 1}
    }' --accountId $NFT_CONTRACT_ID --deposit 0.1 --gas 100000000000000