build_wallet:   
    cargo build -p smartwallet --release --target wasm32-wasi

translate_wasm:
    wasi2ic ./target/wasm32-wasi/release/smartwallet.wasm smartwallet.wasm

install_wallet:
    dfx canister install --mode reinstall --wasm smartwallet.wasm smartwallet