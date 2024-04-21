
create_all_canisters:
    dfx canister create --all

deploy_steward:
    dfx deploy steward --argument '("regtest")'

deploy_os:
    STEWARD_CANISTER=$(dfx canister id steward) && dfx deploy os --argument "(record { network = variant { regtest }; steward_canister =  principal \"$STEWARD_CANISTER\"; })"

    
build_wallet:   
    cargo build -p smartwallet --release --target wasm32-wasi

translate_wasm:
    wasi2ic ./target/wasm32-wasi/release/smartwallet.wasm smartwallet.wasm

install_wallet:
    #!/usr/bin/env bash
    # set -euxo pipefail
    STEWARD_CANISTER=$(dfx canister id steward)
    echo $STEWARD_CANISTER
    dfx canister install --wasm smartwallet.wasm smartwallet --mode reinstall --argument "(record { network = variant { regtest }; steward_canister = principal \"$STEWARD_CANISTER\" })"

deploy_wallet:
    #!/usr/bin/env bash
    # set -euxo pipefail
    STEWARD_CANISTER=$(dfx canister id steward)
    dfx deploy smartwallet --argument "(record { network = variant { regtest }; steward_canister = principal \"$STEWARD_CANISTER\" })" --mode reinstall

