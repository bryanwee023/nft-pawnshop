Nft Pawnshop
==================

A [Near smart contract] written in [Rust] for an app that uses NFTs to facilitate P2P lending. NFT owners can use their NFTs as collateral to request for loans. Lenders can accept loan conditions at an agreed interest rate and default date.


Prerequisites
===========
1. Make sure you've installed [Node.js] ≥ 12
2. Install [Rust] and [cargo]
3. Install [near-cli] globally. This will be useful for deployment and manual testing.

   ```   
    yarn install --global near-cli
   ```

Quick Start
===========

## Step 1. Set up

Set up your account id as a environment variable. This way, it's easier to copy and paste the subsequent command.

    export PAWNSHOP_ID="YOUR_ACCOUNT_NAME"

## Step 2. Deploy and initialise the contract

    yarn dev-deploy $PAWNSHOP_ID

Running this command will prompt you to log into the NEAR account you intend to deploy the contract onto.
The command then builds the contract, deploys and initialises it.

## Step 3. Interact with the contract

First, let's try to list an nft as a collateral for a loan. Log into a separate testnet account and set it as an environment variable.

    near login
    export USER_ID="YOUR_OTHER_ACCOUNT_NAME"
    
To pawn an nft, the user must approve the contract in transferring the nft to be pawned. 

Then, the user can list the nft for pawning as such.

    near call $PAWNSHOP_ID offer_pawn '{
        "nft_contract_id": "'$NFT_CONTRACT'", 
        "token_id": "'Token4'", 
        "loan_conditions": {
            "loan_value":"1000000000000000000000000", 
            "interest":10, 
            "duration":86400000000000 
        }
    }' --deposit 0.01 --accountId $USER_ID --gas=300000000000000

This lists our nft as a collateral for a 1 day loan of 1 near, with 10% interest rate.
   
**Note**: To mint NFTs for manual testing purposes, we can make use of the __nft-contract/nft-scripts__ subfolder.

To learn how to accept pawn offers, repay loans, and collect collateral, read the documentation (TBD)
     
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

