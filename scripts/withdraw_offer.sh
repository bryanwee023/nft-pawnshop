#!/usr/bin/env bash
cd "`dirname $0`"
. ./config.sh

near call $PAWNSHOP_ID withdraw_offer '{ "nft_contract_id": "'$NFT_CONTRACT_ID'", "token_id": "'$TOKEN_ID'" }' \
     --accountId $BORROWER_ID --gas=$MORE_GAS