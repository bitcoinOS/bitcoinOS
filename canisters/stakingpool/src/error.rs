use candid::CandidType;

#[derive(Debug, thiserror::Error, CandidType)]
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
