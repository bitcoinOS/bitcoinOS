use candid::CandidType;

#[derive(Debug, CandidType, thiserror::Error)]
pub enum Error {
    #[error("Wallet canister {wallet_canister:?} already exists")]
    WalletAlreadyExists { wallet_canister: String },
    #[error("Wallet canister {0:?} not found")]
    WalletNotFound(String),
    #[error("Create wallet canister failed: {msg:?}")]
    CreateCanisterFailed { msg: String },
    #[error("Stable write error: {msg:?}")]
    StableWriteError { msg: String },
    #[error("Stable set error: {msg:?}")]
    StableSetError { msg: String },
    #[error("Staking pool canister {staking_pool_id:?} already exists")]
    StakingPoolAlreadyExists { staking_pool_id: String },
    #[error("Create wallet canister failed: {msg:?}")]
    GetStakingPoolAddressFailed { msg: String },
    #[error("Confirm Staking Record error: {0:?}")]
    ConfirmStakingError(String),
    #[error("Redeemed Staking Record error: {0:?}")]
    RedeemedStakingError(String),
    #[error("Candid encode error: {0:?}")]
    CandidEncodeError(String),
    #[error("UnAuthorized: {0:?}")]
    UnAuthorized(String),
    #[error("Unknown error")]
    Unknown,
}
