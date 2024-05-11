
create_all_canisters:
    dfx canister create --all


deploy_os:
    dfx deploy os --argument "(record { network = variant { regtest }; steward_canister =  principal \"aaaaa-aa\"; })" 

build_staking:
    cargo build -p stakingpool --release --target wasm32-unknown-unknown

build_wallet:   
    cargo build -p smartwallet --release --target wasm32-unknown-unknown 

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

deploy_frontend:
    dfx deploy bitcoinOS_frontend 

create_staking_pool:
    dfx canister call os create_staking_pool_canister '(record { duration_in_day = 30:nat64; name = "staking pool test"; description = "a staking pool with 10 annual interest rate for a year"; annual_interest_rate = 10:nat16 })'

mint_cycles:
    #!/usr/bin/env bash
    # set -euxo pipefail
    wallet=$(dfx identity get-wallet)
    dfx ledger fabricate-cycles --t 2000 --canister $wallet
    dfx wallet balance

deposit_os:
    dfx canister deposit-cycles 20000000000000 os