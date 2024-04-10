use bitcoin::EcdsaSighashType;

/// The default bitcoin fee in millisatoshi per byte
pub const DEFAULT_FEE_MILLI_SATOSHI: u64 = 2000;
pub const DUST_AMOUNT_SATOSHI: u64 = 546;

/// The fees for the various bitcoin endpoints.
pub const GET_BALANCE_COST_CYCLES: u64 = 100_000_000;
pub const GET_UTXOS_COST_CYCLES: u64 = 10_000_000_000;
pub const GET_CURRENT_FEE_PERCENTILES_CYCLES: u64 = 100_000_000;
pub const SEND_TRANSACTION_BASE_CYCLES: u64 = 5_000_000_000;
pub const SEND_TRANSACTION_PER_BYTE_CYCLES: u64 = 20_000_000;

// The fee for the `sign_with_ecdsa` endpoint using the test key.
pub const SIGN_WITH_ECDSA_COST_CYCLES: u64 = 25_000_000_000;

/// The default Signature Hash Type for bitcoin transactions
pub const SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;
