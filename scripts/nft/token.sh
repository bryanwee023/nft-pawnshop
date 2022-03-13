#!/usr/bin/env bash
cd "`dirname $0`"
. ./../config.sh

near view $NFT_CONTRACT_ID nft_token "{\"token_id\": \"$TOKEN_ID\"}"