deploy_os:
    dfx canister create os

deploy_steward:    
    dfx canister create steward --argument '("regtest")'

create_wallet:
    dfx canister create smartwallet

build_wallet:   
    cargo build -p smartwallet --release --target wasm32-wasi

translate_wasm:
    wasi2ic ./target/wasm32-wasi/release/smartwallet.wasm smartwallet.wasm

install_wallet:
    dfx canister install --mode reinstall --wasm smartwallet.wasm smartwallet --argument '("regtest", "replace by steward canister id", "ecdsa_key")'

