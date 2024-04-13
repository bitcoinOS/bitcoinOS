deploy_os:
    dfx deploy os

deploy_steward:
    dfx deploy steward --argument '("regtest")'

build_wallet:   
    cargo build -p wallet --release --target wasm32-wasi

translate_wasm:
    wasi2ic ./target/wasm32-wasi/release/wallet.wasm wallet.wasm

install_wallet:
    export STEWARD_CANISTER=$(dfx canister id steward)
    echo ${STEWARD_CANISTER}
    dfx canister install --mode reinstall --wasm wallet.wasm wallet --argument '(record { network = "regtest"; steward_canister = "${STEWARD_CANISTER}"; key_name = "ecdsa_key" })'