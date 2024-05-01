pub const DEFAULT_CYCLES_PER_CANISTER: u128 = 2_000_000_000_000;

pub const METADATA_SIZE: u64 = 128;

pub const WALLET_WASM: &[u8] =
    std::include_bytes!("./../../../target/wasm32-unknown-unknown/release/smartwallet.wasm");

// TODO: Fix wasm
pub const STAKING_POOL_WASM: &[u8] =
    std::include_bytes!("./../../../target/wasm32-unknown-unknown/release/stakingpool.wasm");
