use candid::CandidType;
use serde::Deserialize;
#[derive(Debug, CandidType, thiserror::Error)]
pub enum Error {
     
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




#[derive(Debug, thiserror::Error, CandidType,Deserialize)]
pub enum StakingError {
    #[error("No Authorize to {0:?}")]
    UnAuthorized(String),

    #[error("Failed to create wallet {0:?}")]
    CreateWalletError(String),

    #[error("Wallet {0:?} already exists")]
    WalletAlreadyExists(String),

    #[error("Bitcoin address {0:?} network is unmatched")]
    BitcoinAddressNetworkUnmatch(String),

    #[error("Invalid bitcoin address: {0:?}")]
    InvalidBitcoinAddress(String),

    #[error("Append redeem log error: {0:?}")]
    AppendRedeemLogError(String),

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Staking record {0:?} already exists")]
    StakingRecordAlreadyExists(String),

    #[error("Staking record {0:?} not found")]
    StakingRecordNotFound(String),

    #[error("Network is unmatched")]
    InvalidNetwork,

    #[error("Redemption not allowed")]
    RedemptionNotAllowed,
}

impl From<wallet::error::Error> for StakingError {
    fn from(value: wallet::error::Error) -> Self {
        StakingError::CreateWalletError(value.to_string())
    }
}
