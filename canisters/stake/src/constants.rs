use candid::Principal;

pub const DEFAULT_CYCLES_PER_CANISTER: u64 = 300_000_000_000;
/// Default cycles per dbank is 20T
pub const DEFAULT_CYCLES_PER_DBANK: u64 = 1_000_000_000_000;

pub const METADATA_SIZE: u64 = 128;

/// The maximum number of wallets a user can create
pub const MAX_WALLET_PER_USER: usize = 1;

pub const SIWB_NAME: &str = "siwb";

//86400
pub const REWARD_PERIOD: u64 = 86400 * 1000000000;

pub const PRINCIPAL_MIN: Principal = Principal::from_slice(&[]);
pub const PRINCIPAL_MAX: Principal = Principal::from_slice(&[255; 29]);

pub const DBANK_WASM: &[u8] =
    std::include_bytes!("./../../../target/wasm32-unknown-unknown/release/dbank.wasm");

pub const STAKING_POOL_WASM: &[u8] =
    std::include_bytes!("./../../../target/wasm32-unknown-unknown/release/stakingpool.wasm");
