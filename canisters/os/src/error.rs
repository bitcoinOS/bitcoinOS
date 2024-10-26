use candid::{CandidType, Principal};

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

    #[error("Txid: {0:?} staknig record already exists")]
    StakingRecordAlreadyExists(String),

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

    #[error("Canister module:{canister_name:?} has exist")]
    CanisterAlreadyExists { canister_name: String },

    #[error("Canister module:{canister_name:?} not exist")]
    CanisterNotExists { canister_name: String },

    #[error("User :{user_id:?} has exist")]
    UserExists { user_id: Principal },
    #[error("User :{user_id:?} has exist")]
    UserNotExists { user_id: Principal },

    #[error("User :{user_id:?} has invited code")]
    InvitedCodeHasExists { user_id: Principal },

    #[error("invited code:{code:?} has exist")]
    InviteCodeNotExists { code: String },

    #[error("Last Reward at {time:?}")]
    RewardTimeIn24Hour { time: u64 },

    #[error("invite code:{code:?} has exist")]
    InviteCodeHasExists { code: String },

    #[error("invite code:{code:?} add error")]
    InviteCodeError { code: String },

    #[error("message:{message:?}")]
    UnvalidParam { message: String },

    #[error("Unknown error")]
    Unknown,

    //bidn wallt error
    #[error("bind wallet too many")]
    BindWalletLimited(),
    #[error("wallet has binded: {0:?}")]
    BindWalletError(String),

    #[error("has not bind wallet : {0:?}")]
    UnBindWallet(String),
}
