#!/usr/bin/env bash
cd "`dirname $0`"
. ./config.sh

near call $PAWNSHOP_ID repay_loan '{ "pawn_id": "'$NFT_CONTRACT_ID'.'$TOKEN_ID'" }' \
    --deposit $BORROWER_REPAYMENT --accountId $BORROWER_ID --gas=$MORE_GAS