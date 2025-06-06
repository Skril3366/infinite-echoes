# Contracts

## Security

1. Use proof of stake to disallow DDOS attack for creating many vaults (for creating a vault a wallet must have required amount of TON)
2. Use rate limiting, to disallow DDOS attack on creating many pieces for a given vault

## Setup

For testing purposes create the following variables in .env file
```
WALLET_MNEMONIC="..."
WALLET_VERSION=v4
TON_NETWORK=testenet
```

## Project structure

-   `contracts` - source code of all the smart contracts of the project and their dependencies.
-   `wrappers` - wrapper classes (implementing `Contract` from ton-core) for the contracts, including any [de]serialization primitives and compilation functions.
-   `tests` - tests for the contracts.
-   `scripts` - scripts used by the project, mainly the deployment scripts.

## How to use

### Build

`npx blueprint build` or `yarn blueprint build`

### Test

`npx blueprint test` or `yarn blueprint test`

### Deploy or run another script

`npx blueprint run` or `yarn blueprint run`

### Add a new contract

`npx blueprint create ContractName` or `yarn blueprint create ContractName`
