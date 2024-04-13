use candid::CandidType;

#[derive(Debug, thiserror::Error, CandidType)]
pub enum StewardError {
    #[error("{0:?} ECDSA key already exists")]
    ECDSAKeyAlreadyExists(String),

    #[error("{0:?} ECDSA key not found")]
    ECDSAKeyNotFound(String),

    #[error("Failed to update ECDSA key")]
    ECDSAKeyUpdateError,

    #[error("Failed to init network")]
    NetworkAlreadyExists,

    #[error("Error from base: {0:?}")]
    BaseError(String),
}

impl From<base::error::Error> for StewardError {
    fn from(value: base::error::Error) -> Self {
        Self::BaseError(value.to_string())
    }
}
