use candid::CandidType;

#[derive(Debug, thiserror::Error, CandidType)]
pub enum DBankError {
    #[error("{0:?} ECDSA key already exists")]
    ECDSAKeyAlreadyExists(String),

    #[error("{0:?} ECDSA key not found")]
    ECDSAKeyNotFound(String),

    #[error("Failed to update ECDSA key")]
    ECDSAKeyUpdateError,

    #[error("{0:?} Public key not found")]
    PublicKeyNotFound(String),

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

    #[error("Append transfer log error: {0:?}")]
    AppendTransferLogError(String),

    #[error("Exceeded max recipient {0:?}")]
    ExceededMaxRecipientError(u8),
    // #[error("Only support p2pkh sign")]
    // OnlySupportP2pkhSign,
}

impl From<wallet::error::Error> for DBankError {
    fn from(value: wallet::error::Error) -> Self {
        DBankError::WalletError(value.to_string())
    }
}
