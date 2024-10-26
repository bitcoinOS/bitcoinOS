use candid::CandidType;

#[derive(Debug, thiserror::Error, CandidType)]
pub enum WalletError {
    #[error("{0:?} ECDSA key already exists")]
    ECDSAKeyAlreadyExists(String),

    #[error("{0:?} ECDSA key not found")]
    ECDSAKeyNotFound(String),

    #[error("Failed to update ECDSA key")]
    ECDSAKeyUpdateError,

    #[error("Failed to register ECDSA key")]
    RegisterECDSAKeyError,

    #[error("Failed to init network")]
    NetworkAlreadyExists,

    #[error("No Authorize to {0:?}")]
    UnAuthorized(String),

    #[error("Failed to create wallet {0:?}")]
    CreateWalletError(String),

    #[error("Error {0:?} when use wallet package")]
    WalletError(String),

    #[error("Error {0:?} when call Steward")]
    StewardCallError(String),

    #[error("Wallet {0:?} not found")]
    WalletNotFound(String),

    #[error("Wallet {0:?} already exists")]
    WalletAlreadyExists(String),

    #[error("Staking record {0:?} already exists")]
    StakingRecordAlreadyExists(String),

    #[error("Staking record {0:?} not found")]
    StakingRecordNotFound(String),

    #[error("Staking record {0:?} can't update")]
    StakingRecordCantUpdate(String),

    #[error("Call staking pool register staking record error: {0:?}")]
    RegisterStakingRecordError(String),

    #[error("Call staking pool sync staking record error: {0:?}")]
    SyncStakingRecordError(String),

    #[error("Append transfer log error: {0:?}")]
    AppendTransferLogError(String),

    // #[error("Insufficient funds")]
    // InsufficientFunds,
    #[error("Exceeded max recipient {0:?}")]
    ExceededMaxRecipientError(u8),

    #[error("Only support p2pkh sign")]
    OnlySupportP2pkhSign,
}

impl From<wallet::error::Error> for WalletError {
    fn from(value: wallet::error::Error) -> Self {
        WalletError::WalletError(value.to_string())
    }
}
