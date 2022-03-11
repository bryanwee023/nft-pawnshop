#!/usr/bin/env bash

# Near addresses
readonly PAWNSHOP_ID="pawnshop-dev.testnet"
readonly BORROWER_ID="bryanwee.testnet"
readonly BROKER_ID="bryanwee2.testnet"
readonly NFT_CONTRACT_ID="nftcontract2.testnet"
readonly TOKEN_ID="Token4"

# Loan conditions
readonly LOAN_VALUE_YOCTO=1000000000000000000000000 # In YoctoNEAR
readonly INTEREST=10
readonly DURATION=86400000000000

# Payment values
readonly BROKER_DEPOSIT="1.1" # In NEAR
readonly BORROWER_REPAYMENT="1.1" # In NEAR

# Gas
readonly MORE_GAS=300000000000000 # In YoctoNEAR. For function calls that require cross-contract calls.
readonly DEFAULT_GAS=100000000000000 # In YoctoNEAR. For regular function calls.
