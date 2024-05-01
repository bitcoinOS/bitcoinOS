use candid::CandidType;

#[derive(Debug, CandidType, thiserror::Error)]
pub enum Error {
    #[error("Wallet canister {wallet_id:?} already exists")]
    WalletAlreadyExists { wallet_id: String },
    #[error("Create wallet canister failed: {msg:?}")]
    CreateCanisterFailed { msg: String },
    #[error("Stable write error: {msg:?}")]
    StableWriteError { msg: String },
    #[error("Stable set error: {msg:?}")]
    StableSetError { msg: String },
    #[error("Staking pool canister {staking_pool_id:?} already exists")]
    StakingPoolAlreadyExists { staking_pool_id: String },
    #[error("UnAuthorized: {0:?}")]
    UnAuthorized(String),
    #[error("Unknown error")]
    Unknown,
}
