Nft Pawnshop
==================

A [Near smart contract] written in [Rust] for an app that uses NFTs to facilitate P2P lending. NFT owners can use their NFTs as collateral to request for loans. Lenders can accept loan conditions at an agreed interest rate and due date.


Prerequisites
===========
1. Make sure you've installed [Node.js] ≥ 12
2. Install [Rust] and [Cargo]
3. Install **near-cli** globally. This will be used for deployment and manual testing.
  
        yarn install --global near-cli

Quick Start
===========

## Step 1. Set up

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

## Step 2. Deploy the contract

    yarn dev-deploy

The command will deploy and initialiese the contract onto the testnet in development mode. 

## Step 3. Interact with the contract

First, let's try to list an nft as a collateral for a loan by:

1. Approving the account to transfer the nft of interest.

        ./scripts/nft/approve.sh


2. Then, call contract code to list the pawn offer

        yarn offer-pawn


:bulb: To mint NFTs for manual testing purposes, we can run `scripts/nft/mint.sh`.

To see how to accept pawn offers, repay loans, and collect collateral, read the documentation
     
Troubleshooting
===========

On Mac M1, cross-compilation is not supported on Apple ARM yet. This might cause problems when trying to run tests. Please see this [issue](https://github.com/near/nearcore/issues/3803). To circumvent this, do switch to a x86-64 toolchain.

Install the toolchain if you've not done so. 

     rustup toolchain install stable-x86_64-apple-darwin

Set the toolchain as the default. 

     rustup default stable-x86_64-apple-darwin


  [Near smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Node.js]: https://nodejs.org/en/download/package-manager/
  [Rust]: https://www.rust-lang.org/
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

