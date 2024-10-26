pub mod request;
pub mod response;
use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, Satoshi};
use ic_cdk_timers::TimerId;
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

use crate::constants::{
    self, INTERVAL_CREATE_STAKING_RECORD_IN_SECONDS, INTERVAL_SAVE_POINT_IN_SECONDS,
    INTERVAL_UPDATE_BTC_PRICE_IN_SECONDS, INTERVAL_UPDATE_LEADERBOARD_IN_SECONDS,
};
use std::borrow::Cow;
use wallet::domain::{
    reward::{RewardMode, RewardType},
    staking::StakingType,
    user::UserType,
};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Metadata {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub os_canister: Principal,
    pub siwb_canister: Option<Principal>,
    // pub period: u64,
    // pub point_per_sat: u64,
    // pub point_decimal: u64,
    pub updated_time: u64,
}

impl Default for Metadata {
    fn default() -> Self {
        let network = BitcoinNetwork::Regtest;
        Self {
            network,
            steward_canister: Principal::anonymous(),
            os_canister: Principal::anonymous(),
            siwb_canister: None,
            // period: DEFAULT_TIME_PER_PERIOD,
            // point_per_sat: DEFAULT_POINT_PER_SAT,
            // point_decimal: POINT_DECIMAL,
            updated_time: 0,
        }
    }
}

impl Storable for Metadata {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct PointRecord {
//     pub network: BitcoinNetwork,
//     pub staker: Principal,
//     pub wallet: String,
//     pub stake_type: StakingType,
//     // pub stake_pool_canister: CanisterId,
//     pub actual_amount: Satoshi,
//     //total points
//     pub points: u64,
//     pub updated_time: u64,
// }

// impl PointRecord {
//     pub fn can_update(&self, other: &Self) -> bool {
//         self.staker == other.staker
//             // && self.stake_pool_canister == other.stake_pool_canister
//             && self.network == other.network
//             && self.updated_time < other.updated_time
//     }
// }

// impl Storable for PointRecord {
//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         std::borrow::Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PriceRecord {
    pub message: String,
    pub price: u64,
    pub updated_time: u64,
}

impl PriceRecord {
    pub fn can_update(&self, other: &Self) -> bool {
        self.price != other.price
    }
}

impl Storable for PriceRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_default()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for PriceRecord {
    fn default() -> Self {
        Self {
            message: "".to_string(),
            price: 0,
            updated_time: 0,
        }
    }
}
//reward
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct RewardRecord {
    pub user_id: Principal,
    pub user_type: UserType,
    pub rank: u64,
    pub total_point: u64,
    pub network: BitcoinNetwork,
    pub update_time: u64,
    pub create_time: u64,
    // pub max_box_id: u64,
}
impl Storable for RewardRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_default()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for RewardRecord {
    fn default() -> Self {
        Self {
            user_id: Principal::anonymous(),
            total_point: 0,
            rank: 0,
            update_time: 0,
            create_time: 0,
            network: BitcoinNetwork::Regtest,
            user_type: UserType::II, // max_box_id: 1,
        }
    }
}

//box reward
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct BoxRewardRecord {
    pub user_id: Principal,
    pub network: BitcoinNetwork,

    // pub has_invited_reward:bool,
    pub total_box_count: u64,
    pub og_count: u64,
    pub fund_count: u64,
    pub boost_card_count: u64,
    pub box_point: u64,
    pub unopen_box_count: u64,
    pub max_box_id: u64,
    pub update_time: u64,
    pub create_time: u64,
    // pub max_box_id: u64,
}

impl Storable for BoxRewardRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_default()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for BoxRewardRecord {
    fn default() -> Self {
        Self {
            user_id: Principal::anonymous(),
            max_box_id: 0,
            og_count: 0,
            fund_count: 0,
            boost_card_count: 0,
            total_box_count: 0,
            box_point: 0,
            unopen_box_count: 0,
            update_time: 0,
            create_time: 0,
            network: BitcoinNetwork::Regtest,
            // max_box_id: 1,
        }
    }
}

//invite reward
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InviteRewardRecord {
    pub user_id: Principal,

    pub network: BitcoinNetwork,

    // invited points
    pub total_invite_count: u64,
    pub avalable_invite_count: u64,
    pub invited_points: u64,
    pub invite_user_id: Option<Principal>,
    pub invite_status: InviteStaus,
    pub update_time: u64,
    pub create_time: u64,
    // pub max_invite_index:u64,
}

impl Storable for InviteRewardRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_default()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for InviteRewardRecord {
    fn default() -> Self {
        Self {
            user_id: Principal::anonymous(),
            // invited points
            invited_points: 0,
            invite_user_id: None,
            invite_status: InviteStaus::UnInvited,
            total_invite_count: 0,
            avalable_invite_count: 0,
            // max_invite_index:0,
            update_time: 0,
            create_time: 0,
            network: BitcoinNetwork::Regtest,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StakeRewardRecord {
    pub user_id: Principal,
    pub network: BitcoinNetwork,
    pub stake_type: StakingType,
    pub stake_amount: Satoshi,
    pub stake_point: u64,
    // pub stake_point_decimal:Option<u8>,
    pub last_stake_reward_at: u64,
    //add 0921
    pub last_stake_ammount: Option<u64>,
    pub last_stake_at: u64,
    pub update_time: u64,
    pub create_time: u64,
    pub stake_point_update_at: Option<u64>,
}

impl Storable for StakeRewardRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_default()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for StakeRewardRecord {
    fn default() -> Self {
        Self {
            user_id: Principal::anonymous(),
            last_stake_reward_at: 0,
            update_time: 0,
            create_time: 0,
            network: BitcoinNetwork::Regtest,
            stake_amount: 0,
            stake_point: 0,
            stake_type: StakingType::OSWallet,
            last_stake_at: 0,
            last_stake_ammount: Some(0),
            stake_point_update_at: Some(0),
            // stake_point_decimal:None,
        }
    }
}

//reward
// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct BoxRewardDetail {
//     pub user_id: Principal,
//     pub point: u64,
//     pub og_count: u64,
//     pub fund_count: u64,
//     pub boost_card_count: u64,
//     pub box_id: u64,
//     pub create_time: u64,
// }

// impl Storable for BoxRewardDetail {
//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         std::borrow::Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap_or_default()
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

// impl Default for BoxRewardDetail {
//     fn default() -> Self {
//         Self {
//             user_id: Principal::anonymous(),
//             point: 0,
//             og_count: 0,
//             fund_count: 0,
//             boost_card_count: 0,
//             box_id: 0,
//             create_time: 0,
//         }
//     }
// }

//box
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct BoxRecord {
    pub user_id: Principal,
    pub box_id: u64,
    pub box_status: BoxStatus,
    pub box_type: RewardType,
    pub point: u64,
    pub og_count: u64,
    pub fund_count: u64,
    pub boost_card_count: u64,
    pub create_time: u64,
    pub open_time: u64,
    pub reward_mode: Option<RewardMode>,
}

impl Storable for BoxRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_default()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for BoxRecord {
    fn default() -> Self {
        Self {
            user_id: Principal::anonymous(),
            box_id: 0,
            box_status: BoxStatus::Close,
            box_type: RewardType::Stake,
            point: 0,
            og_count: 0,
            fund_count: 0,
            boost_card_count: 0,
            create_time: 0,
            open_time: 0,
            reward_mode: Some(RewardMode::Random),
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoxStatus {
    Open,
    Close,
}

// // invite record
// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct InviteRecord {
//     pub invite_user_id: Principal,
//     pub invited_user_id: Principal,
//     pub invite_status: InviteStaus,
//     pub create_time: u64,
// }
#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum InviteStaus {
    Init,
    Confirmed,
    UnInvited,
}

// impl Storable for InviteRecord {
//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         std::borrow::Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap_or_default()
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

// impl Default for InviteRecord {
//     fn default() -> Self {
//         Self {
//             invite_user_id: Principal::anonymous(),
//             invited_user_id: Principal::anonymous(),
//             invite_status: InviteStaus::Init,
//             create_time: 0,
//         }
//     }
// }

// user status

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct UserStatusRecord {
//     pub user_id: Principal,
//     pub max_box_id: u64,
//     // pub last_reward_at: u64,
//     pub invite_user_id: Principal,
//     pub invite_status: InviteStaus,
//     pub update_time: u64,
//     pub create_time: u64,
// }
// #[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
// pub enum InviteStaus {
//     UnInvited,
//     Init,
//     Confirmed,
// }

// impl Storable for UserStatusRecord {
//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         std::borrow::Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap_or_default()
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

// impl Default for UserStatusRecord {
//     fn default() -> Self {
//         Self {
//             user_id: Principal::anonymous(),
//             invite_user_id: Principal::anonymous(),
//             invite_status: InviteStaus::Init,
//             create_time: 0,
//             update_time: 0,
//             max_box_id: 0,
//             // last_reward_at: 0,
//         }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CandidType, Deserialize)]
pub struct BoxKey {
    pub user_id: Principal,
    pub box_id: u64,
}

impl Storable for BoxKey {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

// #[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
// pub struct UserStakePool{
//    pub user: Principal,
//    pub stake_pool: CanisterId
// }

// impl Storable for UserStakePool {
//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         std::borrow::Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

// Staking pool info will be stored in stable storage
// #[derive(Debug, CandidType, Deserialize, Clone)]
// pub struct StakingPoolInfo {
//     pub staking_pool_canister: CanisterId,
//     pub bitcoin_address: String,
//     pub name: String,
//     pub description: String,
//     pub network: BitcoinNetwork,
//     pub annual_interest_rate: u16,
//     pub duration_in_day: u64,
//     pub os_canister: CanisterId,
//     pub created_at: u64,
// }

// /// For a type to be used in Stable storage like `StableBtreeMap`, it need to implement the `Storable` trait,
// /// which specifies how the type can be serialized/deserialized.
// impl Storable for StakingPoolInfo {
//     fn from_bytes(bytes: Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }

//     fn to_bytes(&self) -> Cow<[u8]> {
//         Cow::Owned(Encode!(self).unwrap())
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ConfigSetting {
    pub sat_per_point: u64,
    pub reward_period: u64,
    pub base_point_per_box: u64,
    pub max_rand_box_point: u64,
    pub invite_point_per_user: u64,
    pub invite_point_rate: u64,
    pub update_time: u64,
    pub create_time: u64,
    // pub max_box_id: u64,
}

impl Storable for ConfigSetting {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_default()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for ConfigSetting {
    fn default() -> Self {
        Self {
            sat_per_point: constants::DEFAULT_SAT_PER_POINT,
            update_time: 0,
            create_time: 0,
            max_rand_box_point: constants::MAX_BOX_RAND_POINT,
            reward_period: constants::REWARD_PERIOD,
            base_point_per_box: constants::BOX_BASE_POINT,
            invite_point_per_user: constants::INVITE_POINT_PER_USER,
            invite_point_rate: constants::INVITE_POINT_RATE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CandidType, Deserialize)]
pub struct RankKey {
    pub user_id: Principal,
    pub rank: u64,
}

impl Storable for RankKey {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct LeaderBoardStatus {
    pub total_user: u64,
    pub total_user_gt_zero: u64,
    pub min_rank: u64,
    pub max_point: u64,
    pub min_point: u64,
    pub update_time: u64,
}

impl Storable for LeaderBoardStatus {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_default()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for LeaderBoardStatus {
    fn default() -> Self {
        Self {
            total_user_gt_zero: 0,
            update_time: 0,
            total_user: 0,
            min_rank: 0,
            max_point: 0,
            min_point: 0,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct TimerSettings {
    pub save_point_in_secs: u64,
    pub create_staking_record_in_secs: u64,
    pub update_btc_price_in_secs: u64,
    pub update_leaderboard_in_secs: u64,
}

impl Default for TimerSettings {
    fn default() -> Self {
        Self {
            save_point_in_secs: INTERVAL_SAVE_POINT_IN_SECONDS,
            create_staking_record_in_secs: INTERVAL_CREATE_STAKING_RECORD_IN_SECONDS,
            update_btc_price_in_secs: INTERVAL_UPDATE_BTC_PRICE_IN_SECONDS,
            update_leaderboard_in_secs: INTERVAL_UPDATE_LEADERBOARD_IN_SECONDS,
        }
    }
}

impl Storable for TimerSettings {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_default()
    }

    const BOUND: Bound = Bound::Unbounded;
}

/// The information of a wallet
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct UserStat {
    pub day: u64,
    pub user_count: u128,
}

impl Storable for UserStat {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for UserStat {
    fn default() -> Self {
        Self {
            day: 0,
            user_count: 0,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct NftRewardRecord {
    pub user_id: Principal,
    pub network: BitcoinNetwork,
    pub staked_nft_count: u64,
    pub boost_rate: u64,
    pub stake_nft_point: u64,
    pub stake_nft_point_update_at: u64,
    pub update_time: u64,
    pub create_time: u64,
}

impl Storable for NftRewardRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_default()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for NftRewardRecord {
    fn default() -> Self {
        Self {
            user_id: Principal::anonymous(),
            staked_nft_count: 0,
            boost_rate: 100,
            stake_nft_point: 0,
            stake_nft_point_update_at: 0,
            update_time: 0,
            create_time: 0,
            network: BitcoinNetwork::Testnet,
        }
    }
}
