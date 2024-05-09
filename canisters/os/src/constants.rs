use candid::Principal;

pub const DEFAULT_CYCLES_PER_CANISTER: u64 = 100_000_000_000;

pub const METADATA_SIZE: u64 = 128;

pub const PRINCIPAL_MIN: Principal = Principal::from_slice(&[]);
pub const PRINCIPAL_MAX: Principal = Principal::from_slice(&[255; 29]);

pub const WALLET_WASM: &[u8] =
    std::include_bytes!("./../../../target/wasm32-unknown-unknown/release/smartwallet.wasm");

pub const STAKING_POOL_WASM: &[u8] =
    std::include_bytes!("./../../../target/wasm32-unknown-unknown/release/stakingpool.wasm");
