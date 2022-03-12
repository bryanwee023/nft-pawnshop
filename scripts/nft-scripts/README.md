Nft Scripts
==================

NFT related scripts manual testing.

## Manual Testing on Testnet

To aid manual testing on the testnet, this folder provides NEAR-CLI commands to interact with a deployed nft-contract.
Set up global parameters (e.g. nft contract address, receiver address, token id) via **config.sh**.

**Note**: We can utilise the pre-deployed `nftcontract2.testnet` for testing purposes here.

| Script | Description |
| ------------- | ------------- |
| `./mint.sh`  | Mints token |
| `./token.sh`  | Gets token information  |
| `./transfer.sh`  | Transfers token |
| `./approve.sh`  | Approves token for receiver's use |

