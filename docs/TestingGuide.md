# Testing Guide

The project only supports unit tests and manual testing (with near-cli).
Simulation tests will be added after cross-contract compilation is supported in Apple M1 chips. See [this issue](https://github.com/near/nearcore/issues/3803) for more information.

## Prerequisites
1. Make sure you've installed [Node.js] ≥ 12
2. Install [Rust] and [Cargo]
3. Install **near-cli** globally. This will be used for deployment and manual testing.
  
        yarn install --global near-cli

## Setting up

The scripts to run and interact with the contract all use the parameters in `scripts/config.sh`.

     vim scripts/config.sh

Start by configuring these parameters.

| Name | Description |
| ------------- | ------------- |
| `PAWNSHOP_ID`  | Near address to deploy contract on |
| `BORROWER_ID`  | Near address of borrower / nft owner  |
| `BROKER_ID`  | Near address of broker |
| `NFT_CONTRACT_ID`  | Near address of nft to pawn |
| `TOKEN_ID`  | ID of nft to pawn |

**NOTE**: You'll need the full access keys for `PAWNSHOP_ID`, `BORROWER_ID`, AND `BROKER_ID` to call contract methods later on.

## Deploying the contract

    yarn dev-deploy

The command will deploy and initialiese the contract onto the testnet in development mode. 

## Interacting with the contract

The repository has several yarn scripts one can run to interact with the deployed contract.

| Command | Description |
| ------------- | ------------- |
| `yarn offer-pawn`  | Borrower lists a pawn offer with specified nft and loan conditions |
| `yarn withdraw-offer`  | Borrower withdraws the listed pawn offer  |
| `yarn accept-pawn `  | Broker accepts the listed pawn offer |
| `yarn repay-loan`  | Borrower repays the loan and recollects nft  |
| `yarn liquidate-pawn`  | Broker liquidates the pawn  |

All scripts can have their parameters modified via `scripts/config.sh`

## Nft Stub

Testers might need to use a dummy nft contract. We can use pre-deployed `nftcontract2.near` that implements in accordance to NEAR's [nft contract standards](https://github.com/near-examples/NFT/blob/0ff98d011fbbcec7333666ebc5c7e704297f7cec/nft/src/lib.rs).

To quickly mint an nft for testing purposes:

     ./scripts/nft/mint.sh

To approve the pawnshop id to transfer the nft:

     ./scripts/nft/approve.sh