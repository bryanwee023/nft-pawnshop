Nft Pawnshop
==================

A [Near smart contract] written in [Rust] for an app that uses NFTs to facilitate P2P lending. NFT owners can use their NFTs as collateral to request for loans. Lenders can accept loan conditions at an agreed interest rate and default date.


Prerequisites
===========
1. Make sure you've installed [Node.js] ≥ 12
2. Install [Rust]
3. Install [near-cli] globally. This will be useful for deployment and manual testing.

   ```   
    yarn install --global near-cli
   ```
Quick Start
===========

## Step 1. Set up

Log into the near account you intend to deploy the contract on.

    near login

Set up your account id as a environment variable. This way, it's easier to copy and paste the subsequent command.

    export PAWNSHOP_ID="YOUR_ACCOUNT_NAME"

## Step 2. Deploy and initialise the contract

    yarn build
    near deploy --wasmFile out/main.wasm --accountId $PAWNSHOP_ID
    near call $PAWNSHOP_ID new --account_id $PAWNSHOP_ID

The block builds the contract, deploys it onto the testnet, and then initialises it.

## Step 3. Interact with the contract

First, let's try to list an nft as a collateral for a loan. Log into a separate testnet account and set it as an environment variable.

    near login
    export USER_ID="YOUR_OTHER_ACCOUNT_NAME"
    
Before any user can list an nft as a pawn, they will have to pay for storage. To do so:

    near call $PAWNSHOP_ID deposit_for_storage --deposit 0.01   --accountId $USER_ID

Now, to initiate a pawn offer, let's transfer an NFT to the contract with the following message.

    "
       {
          "loan_value": "1000000000000000000000000",
          "interest": 10,
          "duration": 86400000000000
       }
    "
   
This should initiate an offer requesting a 1 day loan of 1 Near, at a 10% interest rate.

To mint NFTs for manual testing purposes, we can make use of the __nft-contract__ subfolder. To transfer, run the following:
     
     
     

  [Near smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Node.js]: https://nodejs.org/en/download/package-manager/
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

