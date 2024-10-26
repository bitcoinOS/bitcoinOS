
## DBank Canister

Each `dbank` canister sets a maximum bitcoin wallet limit, which is currently 10,000.
If the number of bitcoin wallets in a `dbank` canister exceeds the maximum limit, the `os` canister will automatically create a new dbank canister to serve new bitcoin wallet requests.

`dbank` canister creates a bitcoin wallet for each user's `Principal`. A user can only create one bitcoin wallet.

`dbank` canister creates a Bitcoin wallet for each user Principal. A user can only create one bitcoin wallet.

`dbank` canister includes the following main functions:
- Create a Bitcoin wallet with a P2WPKH format address
- Create a Bitcoin wallet with a P2PKH format address
- Get the user's P2WPKH format address
- Get the user's P2PKH format address
- Get the balance of the specified Bitcoin address
- Get all Bitcoin wallets of this dbank canister
- Get the user's Bitcoin public key
- Transfer BTC with a P2WPKH address
- Transfer BTC with a P2PKH address
- Use a P2WPKH address to stake to a specified stake pool
- Use a P2PKH address to stake to a specified stake pool
- Operation log of dbank canister

etc.

see details: [dbank api](./dbank.did)