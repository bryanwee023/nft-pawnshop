#!/usr/bin/env bash
. ./nft-scripts/config.sh

near call example-nft.testnet nft_mint "{
    \"token_id\": \"$TOKEN_ID\", 
    \"receiver_id\": \"$NFT_OWNER_ID\", 
    \"token_metadata\": { 
        \"title\": \"$TOKEN_ID\", 
        \"description\": \"For testing purposes\", 
        \"media\": \"https://bafybeidl4hjbpdr6u6xvlrizwxbrfcyqurzvcnn5xoilmcqbxfbdwrmp5m.ipfs.dweb.link/\", 
        \"copies\": 1}
    }" --accountId $NFT_OWNER_ID --deposit 0.1