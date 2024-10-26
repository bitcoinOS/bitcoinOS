use candid::{CandidType, Principal};
use serde::Deserialize;
#[derive(Debug, thiserror::Error, CandidType, Deserialize)]
pub enum Error {
    #[error("Invalid transaction")]
    InvalidTransaction,

    #[error("{msg}")]
    CreateWalletError { msg: String },

    #[error("IO error")]
    DeserializeError,

    #[error("Call IC error: {0:?}")]
    ICCallError((ic_cdk::api::call::RejectionCode, String)),

    #[error("Invalid principal: {0:?} for a wallet")]
    InvalidPrincipal(Principal),

    #[error("Secp256k1 error: {0:?}")]
    Secp256k1Error(String),

    #[error("Invalid Bitcoin Address: {0:?}")]
    InvalidBitcoinAddress(String),

    #[error("Only support P2PKH sign")]
    OnlySupportP2pkhSign,

    #[error("Only support P2WPKH sign")]
    OnlySupportP2wpkhSign,

    #[error("Bitcoin address unmatch network: {0:?}")]
    BitcoinAddressUnmatchNetwork(String),

    #[error("{0:?} ECDSA key already exists")]
    ECDSAKeyAlreadyExists(String),

    #[error("{0:?} ECDSA key not found")]
    ECDSAKeyNotFound(String),

    #[error("Failed to update ECDSA key")]
    ECDSAKeyUpdateError,

    #[error("Amount is not match with address amount")]
    AmountsAndAddressesMismatch,

    #[error("Transaction and signatures mismatch")]
    TransactionAndSignaturesMismatch,

    #[error("Transaction hash error: {0:?}")]
    TransactionHashError(String),

    #[error("P2wshSigHash error: {0:?}")]
    P2wshSigHashError(String),

    #[error("Transaction amount less than dust")]
    AmountLessThanDust,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Stable write error: {msg:?}")]
    StableWriteError { msg: String },

    #[error("Recipients exceeded max limit {0}")]
    ExceededMaxRecipientError(u8),

    #[error("unkown error: {0:?}")]
    UnkownError(String),
}

impl From<(ic_cdk::api::call::RejectionCode, String)> for Error {
    fn from(e: (ic_cdk::api::call::RejectionCode, String)) -> Self {
        Error::ICCallError(e)
    }
}

impl From<bitcoin::secp256k1::Error> for Error {
    fn from(e: bitcoin::secp256k1::Error) -> Self {
        Error::Secp256k1Error(e.to_string())
    }
}

impl From<bitcoin::address::FromScriptError> for Error {
    fn from(e: bitcoin::address::FromScriptError) -> Self {
        Error::InvalidBitcoinAddress(e.to_string())
    }
}

impl From<bitcoin::address::error::ParseError> for Error {
    fn from(e: bitcoin::address::error::ParseError) -> Self {
        Error::BitcoinAddressUnmatchNetwork(e.to_string())
    }
}

#[derive(Debug, thiserror::Error, CandidType, Deserialize)]
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

    #[error("Txid: {0:?} staknig record already exists")]
    StakingRecordAlreadyExists(String),

    #[error("Staking record {0:?} not found")]
    StakingRecordNotFound(String),

    #[error("Staking pool {0:?} not found")]
    StakingPoolNotFound(String),

    #[error("Network is unmatched")]
    InvalidNetwork,

    #[error("Redemption not allowed")]
    RedemptionNotAllowed,

    #[error("Happens error: {0} when call Steward canister")]
    StewardCallError(String),

    #[error("Happens error: {0} when call OS canister")]
    OsCallError(String),
}

impl From<Error> for StakingError {
    fn from(value: Error) -> Self {
        StakingError::CreateWalletError(value.to_string())
    }
}
