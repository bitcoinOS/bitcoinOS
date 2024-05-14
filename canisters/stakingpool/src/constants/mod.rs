use std::time::Duration;

// The stable store Size for Self Custody keys
pub const SELF_CUSTODY_SIZE: u64 = 1024;

pub const METADATA_SIZE: u64 = 4096;

pub const REDEEM_LOG_SIZE: u64 = 100;

pub const STAKING_RECORD_SIZE: u64 = 100;

/// The daily limit for a wallet in satoshi
pub const DAILY_LIMIET_SATOSHI: u64 = 10_000_000_000;

pub const DAY_IN_MILLISECOND: u64 = 86_400_000;

pub const DAY_IN_NANOSECOND: u64 = 86_400_000_000_000;

/// One hours for staking pool schecdule
pub const ONE_HOURS: Duration = Duration::from_secs(60 * 60);
