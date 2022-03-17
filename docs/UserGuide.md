# User Guide

_What does this smart contract do?_

The smart contract serves as a trustless pawnshop that facilitates NFT-based P2P lending. Users can:

1. Use their NFTs as collateral to borrow NEAR.

2. Lend NEAR to borrowers, and earn from an agreed interest rate.

The smart contract does not have a frontend yet, hence users will have to call contract code directly (e.g. through [near-cli]).

{:toc}

## Quick Start

In P2P lending, users can act as borrowers, or brokers (lenders).

### Requesting for a loan

Borrowers can request for a loan, by offering an nft as collateral. To do so:

1. Approve the pawnshop contract to transfer the nft.
2. Call contract's `offer_pawn` to list the pawn offer.
3. Wait for a broker to accept the pawn offer.
4. Subsequently, repay the loan before the due date.

### Granting a loan

Brokers can grant loans. If loans are not repayed by the due date, brokers can confiscate the collateral nft.

1. Accept a listed pawn offer, paying the contract the loan amount (to be transferred to the borrower).
2. If borrower does not repay by due date, liquidate the pawn and receive the collateral nft.

## Contract API

Examples on how we can call each method using [near-cli] can be found in [here](https://github.com/bryanwee023/nft-pawnshop/blob/main/scripts).

### **Listing a pawn offer**: `offer_pawn`

Lists an pawn offer for brokers to accept. The offer will specify:
1. The nft to be used as collateral
2. The conditions of the loan (amount, interest, & duration)

After listing, the nft will be transferred to the pawnshop for safekeeping. 

_The pawnshop must have previously been approved to transfer the nft._

| Parameters | Description |
| ------------- | ------------- |
| `nft_contract_id`| The contract id of the collateral nft |
| `token_id`  | The token id of the collateral nft  |
| `loan_conditions`  | Conditions of the loan |

### **Withdrawing a pawn offer**: `withdraw_offer`

Withdraws a listed pawn offer (that has not been accepted by a broker).

| Parameters | Description |
| ------------- | ------------- |
| `nft_contract_id`| The contract id of the collateral nft |
| `token_id`  | The token id of the collateral nft  |

### **Accepting a pawn offer**: `withdraw_offer`

Accepts a listed pawn offer. Thereby transferring the borrower the specified amount of NEAR.

| Parameters | Description |
| ------------- | ------------- |
| `pawn_id`| The id of the listed pawn offer |

### **Repaying a loan**: `withdraw_offer`

Repays the loan (with interest) to the pawnbroker. The pawn will be closed, and the nft will be returned to the borrower

| Parameters | Description |
| ------------- | ------------- |
| `nft_contract_id`| The contract id of the collateral nft |
| `token_id`  | The token id of the collateral nft  |

### **Liquidating a pawn**: `liquidate_pawn`

Only for unpaid overdue loans. The broker can close the loan by confiscating the collateral nft.

| Parameters | Description |
| ------------- | ------------- |
| `nft_contract_id`| The contract id of the collateral nft |
| `token_id`  | The token id of the collateral nft  |

### **Reattempt an nft transfer**: `retry_outgoing_transfer`

This is a fallback method not to be used in normal scenarios. It is possible that nft tranfers initiated by the pawnshop contract fail midway (e.g. due to insufficient gas).

To recover from this error, users can retry a previously initiated transfer.

| Parameters | Description |
| ------------- | ------------- |
| `nft_contract_id`| The contract id of the collateral nft |
| `token_id`  | The token id of the collateral nft  |

## FAQ

**What is my pawn id?**

 The id of the pawn is given by `[nft_contract_id].[token_id]`. 
 
 For example, if the pawn offers to use the token `banana_1` from `nft_xyz.id` contract, the corresponding pawn id is `nft_xyz.id.banana_1`.

**How do I specify loan conditions?**

Loan conditions can be specified in JSON like so:

    {
        "loan_value":"[VALUE_YOCTO_NEAR]", 
        "interest":[INTEREST_IN_PERCENT], 
        "duration":[DURATION_IN_NANOSECONDS] 
    }

Here's an example:

    {
        "loan_value":"1000000000000000000000000", 
        "interest":10, 
        "duration": 86400000000000
    }


**I have repayed my loan / liquidated an nft but it has not been transferred to my account?**

It is possible that the nft transfer has failed midway. Try calling `retry_outgoing_transfer()`.

[near-cli]: https://docs.near.org/docs/tools/near-cli