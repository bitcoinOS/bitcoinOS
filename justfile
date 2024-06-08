
create_all_canisters:
    dfx canister create --all


deploy_os:
    dfx deploy os --argument "(record { network = variant { regtest }; steward_canister =  principal \"aaaaa-aa\"; })" 

deploy_os_ic:
    dfx deploy os --ic --argument "(record { network = variant { testnet }; steward_canister =  principal \"aaaaa-aa\"; })" 

generate_did:
    ./scripts/did.sh
    
build_staking:
    cargo build -p stakingpool --release --target wasm32-unknown-unknown

build_wallet:   
    cargo build -p smartwallet --release --target wasm32-unknown-unknown 

build_point:   
    cargo build -p point --release --target wasm32-unknown-unknown 

translate_wasm:
    wasi2ic ./target/wasm32-unknown-unknown/release/smartwallet.wasm smartwallet.wasm

install_wallet:
    #!/usr/bin/env bash
    # set -euxo pipefail
    dfx canister install --wasm smartwallet.wasm smartwallet --mode reinstall --argument "(record { name = \"smartwallet\";  network = variant { regtest }; steward_canister = principal \"aaaaa-aa\" })"

deploy_wallet:
    #!/usr/bin/env bash
    # set -euxo pipefail
    dfx deploy smartwallet --argument '(record { name = "smartwallet"; network = variant { regtest }; steward_canister = principal "aaaaa-aa" })'

deploy_staking:
    #!/usr/bin/env bash
    # set -euxo pipefail
    OS_CANISTER=$(dfx canister id os)
    dfx deploy stakingpool --argument "(record { name = \"stakingpool\"; network = variant { regtest }; os_canister = principal \"$OS_CANISTER\"; description = \"a staking pool with 10 annual interest rate for a year\"; annual_interest_rate = 10; duration_in_millisecond = 86400; })"


deploy_ii:
    dfx deploy internet_identity

deploy_point:
    dfx deploy point  -m reinstall --argument '(record { network = variant { regtest }; os_canister = principal "be2us-64aaa-aaaaa-qaabq-cai"; steward_canister = principal "aaaaa-aa";task_period=60 })'

deploy_frontend:
    dfx deploy bitcoinOS_frontend 

deploy_frontend_ic:
    dfx deploy bitcoinOS_frontend --ic

create_staking_pool:
    dfx canister call os create_staking_pool_canister '(record { duration_in_day = 1:nat64; name = "staking pool test2"; description = "a staking pool with 10 annual interest rate for a year"; annual_interest_rate = 10:nat16 })'

create_staking_pool_ic:
    dfx canister call --ic os create_staking_pool_canister '(record { duration_in_day = 1:nat64; name = "staking pool test"; description = "a staking pool with 10 annual interest rate for a year"; annual_interest_rate = 10:nat16 })'

register_staking_pool_ic:
    dfx canister call --ic os register_staking_pool '(record { duration_in_day = 1:nat64; network = variant { regtest }; name = "staking pool test"; description = "a staking pool with 10 annual interest rate for a year"; annual_interest_rate = 10:nat16; staking_pool_canister = principal "tyg77-iiaaa-aaaah-qdc7q-cai"; staking_pool_address = "mpYWkVyc5wZnj5aPZGBeqQWSeB3vfBwm7Q" })'

create_wallet:
    dfx canister call os create_wallet_canister '("smartwallet")'

mint_cycles:
    #!/usr/bin/env bash
    # set -euxo pipefail
    wallet=$(dfx identity get-wallet)
    dfx ledger fabricate-cycles --t 2000 --canister $wallet
    dfx wallet balance

deposit_os:
    dfx canister deposit-cycles 20_000_000_000_000 os