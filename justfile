
create_all_canisters:
    dfx canister create --all


deploy_os:
    dfx deploy os --argument "(record { network = variant { regtest }; steward_canister =  principal \"2vxsx-fae\"; })" 

build_staking:
    cargo build -p stakingpool --release --target wasm32-unknown-unknown

build_wallet:   
    cargo build -p smartwallet --release --target wasm32-unknown-unknown 

translate_wasm:
    wasi2ic ./target/wasm32-unknown-unknown/release/smartwallet.wasm smartwallet.wasm

install_wallet:
    #!/usr/bin/env bash
    # set -euxo pipefail
    dfx canister install --wasm smartwallet.wasm smartwallet --mode reinstall --argument "(record { name = \"smartwallet\";  network = variant { regtest }; steward_canister = principal \"2vxsx-fae\" })"

deploy_wallet:
    #!/usr/bin/env bash
    # set -euxo pipefail
    dfx deploy smartwallet --argument '(record { name = "smartwallet"; network = variant { regtest }; steward_canister = principal "2vxsx-fae" })'

deploy_staking:
    #!/usr/bin/env bash
    # set -euxo pipefail
    OS_CANISTER=$(dfx canister id os)
    dfx deploy stakingpool --argument "(record { name = \"stakingpool\"; network = variant { regtest }; os_canister = principal \"$OS_CANISTER\"; description = \"a staking pool with 10 annual interest rate for a year\"; annual_interest_rate = 10; duration_in_millisecond = 86400; })"
