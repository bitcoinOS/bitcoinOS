
## OS Canister

OS canister is the backend entrance of BifiPal app, including main functions below:
- user login
- create dbank canister: 
    - There can be multiple dbank canisters, and each dbank canister sets a maximum bitcoin wallet limit, which is currently 10,000.
If the number of bitcoin wallets in a dbank canister exceeds the maximum limit, the OS will automatically create a new dbank canister to serve new bitcoin wallet requests.
- create Bitcoin wallets (including DBank bitcoin wallet (currently used), and SmartWallet independent wallet (suspended)), 
- create staking pools
- user management
- register Module functions
- list all dbank
- list all wallet
- list all staking pool
- count wallet 
- count staking pool
- bind/unbind `Plug` wallet for `ICP`

etc.

see details: [api doc](./os.did) 