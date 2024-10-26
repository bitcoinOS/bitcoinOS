
## SmwartWallet (Suspended)

`smartwallet` canister creates a bitcoin wallet for each user's `Principal`. A user can create multiple bitcoin wallets, but .

`smartwallet` canister is a Bitcoin programmable wallet, a smart contract wallet based on the BIPs specification, and also an Account Abstraction(AA) of Bitcoin.

A `Principal` can create multiple Bitcoin addresses, but each Bitcoin format address can only have one.

smartwallet canister contains the following main functions:
- Get the user's P2WPKH format address
- Get the user's P2WSH format address
- Get the user's P2PKH format address
- Get the balance of the specified Bitcoin address
- Get the UTXOs of the specified address
- Get all Bitcoin addresses of this smartwallet canister
- Get the Bitcoin public key of this wallet
- Transfer BTC with P2WPKH address
- Transfer BTC with P2WSH address
- Transfer BTC with P2PKH address
- Stake to the specified staking pool with P2WSH address
- Stake to the specified staking pool with P2PKH address
- Get the metadata of this wallet, such as the network, creation information, owner, etc.

etc.

see details: [smartwallet api](./smartwallet.did)