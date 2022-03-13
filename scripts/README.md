Near-Cli Scripts
==================

Scripts to quickly run near-cli commands to deploy and interact with the contract.

Setting Parameters
==================

The scripts all use the parameters in `config.sh`.

     vim config.sh

List of Scripts
==================

| Script               | Description |
| -------------------- | ------------- |
| `build.sh`           | Compiles source code and produces the resultant wasm file |
| `deploy.sh [--dev]`  | Deploys and initialises the contract onto `$PAWNSHOP_ID`. Deploys in development mode if specified  |
| `offer_pawn.sh`      | `$BORROWER_ID` offers a pawn for the contract to list (nft must be previouslt approved) |
| `withdraw_offer.sh`  | `$BORROWER_ID` withdraws a previously offered (but unconfirmed) pawn |
| `accept_pawn.sh`     | `$BROKER_ID` accepts an offered pawn, transferring the loan to `$BORROWER_ID` |
| `repay_loan.sh`      | `$BORROWER_ID` repays a previously granted loan to `$BROKER_ID` |
| `liquidate_pawn.sh`  | `$BROKER_ID` collects the collateral nft for the unpaid overdue loan |

The following scripts can be used to interact with a standard nft contract. For testing purposes, there is no need to deploy a new nft contract. Instead, we can use the pre-deployed contract on  `nftcontract2.testnet`.

| Script            | Description |
| ----------------- | ------------- |
| `nft/mint.sh`     | Mint nft and send it to `$BORROWER_ID`  |
| `nft/approve.sh`  | Approve `$PAWNSHOP_ID` to transfer the nft  |