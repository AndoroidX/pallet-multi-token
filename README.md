# pallet-multi-token
Implementation of ERC1155 like token standard as a pallet for Substrate.

## Environment
Environment has been configured in a fork of `substrate-node-template` on a branch `pallet-multi-token`. [Link](https://github.com/AndoroidX/substrate-node-template/tree/pallet-multi-token)
## Config
### `type Balance` 
is a number-like type which is used to store balances.
  
### `type AssetId` 
is a number-like type which is used to store id of an asset.

## Events
The events are straightforward by their names. `Transferred`, `Created`, `Minted`, `ApprovedAll`, `Burned`.

## Errors
```rust
// The lack of permission for this operation
NoPermission,
// There is no approval record for giver operator
NotApproved,
// Not enough balance for the transfer to be done
NotEnoughBalance,
// The account is undefined / does not hold tokens
UndefinedAccount,
// Arithmetic overflow
Overflow,
// The asset is undefined
UndefinedAsset,
```

## Storage
### `Assets`
is a map storage, stores admins (creators and minters) of an asset. Key is an `Config::AssetId` and value is `Config::AccountId` - address of an admin.

### `Accounts`
is a double map storage, stores balances. Keys are `Config::AssetId` and `Config::AccountId`, value is a `Config::Balance` - balance of `AssetId` on account `AccountId`. _Note, empty, but previously active accounts, are stored_

### `Approvals`
is a double map storage, stores approvals. Keys are `Config::AccountId` - owner, `Config::AccountId` - operator. The value is a `bool` - wether assets are approved to be spent by this operator.

### `AssetIdNonce`
is a value, stores the last asset id that was created. Is is used to know what id to assign for the next asset.

## MultiToken trait
`MultiTokenTrait` trait is a public trait that is implemented for the Pallet. It has `safe_transfer`, `set_approve_all` writing functions and `get_balance`, `is_approved_for_all` reader functions. Note that checks for transaction being signed must be performed outside of these functions, __no such checks are performed in these functions__. But `safe_transfer` checks for approvals.

## Mintable trait
`Mintable` train is a public trait that is implemented for the Pallet. It has `create_token` and `mint_tokens` writer functions. Note that checks for transaction being signed must be performed outside of these functions, __no such checks are performed in these functions__.

## Possible attacks and drawbacks
### Traits functions not checking for transaction signature
`MultiTokenTrait` and `Mintable` traits are not performing checks on transaction signature. So other pallets can perform transactions without permission of users. It is needed to keep an eye on pallets that are installed on the runtime and ensure that they are checking transaction signature before calling these functions (`pallet-dex` does check).

## Weights
For the time being weights has not been calculated.