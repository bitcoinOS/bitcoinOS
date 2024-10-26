use candid::{CandidType, Principal};

#[derive(Debug, CandidType, thiserror::Error)]
pub enum Error {
    #[error("nft:{nft_canister:?} id:{nft_id:?} has staked")]
    NFTHasStaked {
        nft_canister: Principal,
        nft_id: u32,
    },

    #[error("nft:{nft_canister:?} id:{nft_id:?} has staked")]
    NFTUnStaked {
        nft_canister: Principal,
        nft_id: u32,
    },

    #[error("UnAuthorized: {0:?}")]
    UnAuthorized(String),

    #[error("bind wallet info error:{0:?}")]
    WalletBindError(String),

    #[error("nft balance info error:{0:?}")]
    NFTBalanceError(String),
    #[error("nft allownce info error:{0:?}")]
    NFTAllownceError(String),

    #[error("nft transfer info error:{0:?}")]
    NFTTransferError(String),

    #[error("nft unstake info error:{0:?}")]
    NFTTUnstakeError(String),

    #[error("Unknown error")]
    Unknown,
}
