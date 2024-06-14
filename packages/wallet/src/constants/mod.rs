use bitcoin::EcdsaSighashType;
use candid::Principal;

// Assume that any amount below this threshold is dust.
pub const DUST_THRESHOLD: u64 = 1_000;

pub const DEFAULT_FEE_MILLI_SATOSHI: u64 = 2000;

// The fee for the `sign_with_ecdsa` endpoint using the test key.
pub const SIGN_WITH_ECDSA_COST_CYCLES: u64 = 25_000_000_000;

/// The default Signature Hash Type for bitcoin transactions
pub const SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;

pub const PRINCIPAL_MIN: Principal = Principal::from_slice(&[]);
pub const PRINCIPAL_MAX: Principal = Principal::from_slice(&[255; 29]);

/// max recipient number when transfer btc
pub const MAX_RECIPIENT_CNT: u8 = 100;
