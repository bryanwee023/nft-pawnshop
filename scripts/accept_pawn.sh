#!/usr/bin/env bash
cd "`dirname $0`"
. ./config.sh

near call $PAWNSHOP_ID accept_pawn '{
    "pawn_id": "'$NFT_CONTRACT_ID'.'$TOKEN_ID'"
}' --deposit $BROKER_DEPOSIT --accountId $BROKER_ID --gas=$MORE_GAS