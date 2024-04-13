
deploy_steward:
    dfx deploy steward --argument '("regtest")'

deploy_os:
    export STEWARD_CANISTER=$(dfx canister id steward)
    dfx deploy os --arguemtn '(record { network = "regest"; steward_canister = "${STEWARD_CANISTER}" })'

create_wallet:
    dfx canister create smartwallet

build_wallet:   
    cargo build -p smartwallet --release --target wasm32-wasi

translate_wasm:
    wasi2ic ./target/wasm32-wasi/release/smartwallet.wasm smartwallet.wasm

install_wallet:
    export STEWARD_CANISTER=$(dfx canister id steward)
    echo ${STEWARD_CANISTER}
    dfx canister install --mode reinstall --wasm smartwallet.wasm smartwallet --argument '(record { network = "regtest"; steward_canister = "${STEWARD_CANISTER}"; key_name = "ecdsa_key" })'