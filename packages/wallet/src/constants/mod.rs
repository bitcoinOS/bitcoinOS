use bitcoin::EcdsaSighashType;
use candid::Principal;

/// The default bitcoin fee in millisatoshi per byte
pub const DUST_AMOUNT_SATOSHI: u64 = 546;
pub const DEFAULT_FEE_MILLI_SATOSHI: u64 = 2000;

// The fee for the `sign_with_ecdsa` endpoint using the test key.
pub const SIGN_WITH_ECDSA_COST_CYCLES: u64 = 25_000_000_000;

/// The default Signature Hash Type for bitcoin transactions
pub const SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;

pub const PRINCIPAL_MIN: Principal = Principal::from_slice(&[]);
pub const PRINCIPAL_MAX: Principal = Principal::from_slice(&[255; 29]);
