
create_all_canisters:
    dfx canister create --all


deploy_os:
    dfx deploy os --argument "(record { network = variant { regtest }; steward_canister =  principal \"aaaaa-aa\"; })"

    
build_wallet:   
    cargo build -p smartwallet --release --target wasm32-wasi

translate_wasm:
    wasi2ic ./target/wasm32-wasi/release/smartwallet.wasm smartwallet.wasm

install_wallet:
    #!/usr/bin/env bash
    # set -euxo pipefail
    dfx canister install --wasm smartwallet.wasm smartwallet --mode reinstall --argument "(record { network = variant { regtest }; steward_canister = principal \"aaaaa-aa\" })"

deploy_wallet:
    #!/usr/bin/env bash
    # set -euxo pipefail
    dfx deploy smartwallet --argument "(record { network = variant { regtest }; steward_canister = principal \"aaaaa-aa\" })" --mode reinstall

