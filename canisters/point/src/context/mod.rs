pub mod memory;

use std::cell::RefCell;

use crate::domain::{
    BoxKey, BoxRecord, BoxRewardRecord, ConfigSetting, InviteRewardRecord, LeaderBoardStatus,
    Metadata, NftRewardRecord, PriceRecord, RewardRecord, StakeRewardRecord, TimerSettings,
    UserStat,
};

use candid::Principal;
use ic_cdk_timers::TimerId;
// use candid::Principal;
use ic_stable_structures::{BTreeMap as StableBTreeMap, Cell as StableCell};
use serde::{Deserialize, Serialize};

use self::memory::Memory;
// use slotmap::SlotMap;

pub type Timestamp = u64;

/// A WalletInfo stable storage has a key with `User Principal` and `Wallet Canister`
// pub type PointRecordsStable = StableBTreeMap<String, PointRecord, Memory>;

pub type RewardRecordStable = StableBTreeMap<Principal, RewardRecord, Memory>;

// pub type BoxRewardDetailStable = StableBTreeMap<BoxKey, BoxRewardDetail, Memory>;

pub type BoxRecordStable = StableBTreeMap<BoxKey, BoxRecord, Memory>;

// pub type UserStatusRecordStable = StableBTreeMap<Principal, UserStatusRecord, Memory>;

pub type BoxRewardStable = StableBTreeMap<Principal, BoxRewardRecord, Memory>;

pub type InviteRewardStable = StableBTreeMap<Principal, InviteRewardRecord, Memory>;

pub type StakeRewardStable = StableBTreeMap<Principal, StakeRewardRecord, Memory>;

pub type UserRankStable = StableBTreeMap<u64, RewardRecord, Memory>;

pub type UserStatStable = StableBTreeMap<u64, UserStat, Memory>;

pub type UserStakeRankStable = StableBTreeMap<u64, StakeRewardRecord, Memory>;
pub type PointAdminStable = StableBTreeMap<Principal, u64, Memory>;

pub type StakedNftRewardStable = StableBTreeMap<Principal, NftRewardRecord, Memory>;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
    pub static TIMER_IDS: RefCell<Vec<TimerId>> = RefCell::new(Vec::new());
    // pub static ADMIN: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_metadata")]
    pub metadata: StableCell<Metadata, Memory>,
    // #[serde(skip, default = "init_stable_point_record")]
    // pub point_records: PointRecordsStable,
    // #[serde(skip, default = "init_stable_next_period")]
    // pub next_period: StableCell<u128, Memory>,
    #[serde(skip, default = "init_stable_btc_price")]
    pub btc_price: StableCell<PriceRecord, Memory>,

    #[serde(skip, default = "init_stable_reward_record")]
    pub reward_record: RewardRecordStable,
    // #[serde(skip, default = "init_stable_box_reward_record")]
    // pub box_reward_record: BoxRewardDetailStable,
    #[serde(skip, default = "init_stable_box_record")]
    pub box_record: BoxRecordStable,
    // #[serde(skip, default = "init_stable_user_status_record")]
    // pub user_status_record: UserStatusRecordStable,
    #[serde(skip, default = "init_stable_config")]
    pub config_setting: StableCell<ConfigSetting, Memory>,

    #[serde(skip, default = "init_stable_box_reward")]
    pub box_reward: BoxRewardStable,

    #[serde(skip, default = "init_stable_invite_reward")]
    pub invite_reward: InviteRewardStable,

    #[serde(skip, default = "init_stable_stake_reward")]
    pub stake_reward: StakeRewardStable,

    #[serde(skip, default = "init_leader_board_status")]
    pub leader_board_status: StableCell<LeaderBoardStatus, Memory>,

    #[serde(skip, default = "init_stable_user_rank")]
    pub user_rank: UserRankStable,

    #[serde(skip, default = "init_stable_timer_settings")]
    pub timer_settings: StableCell<TimerSettings, Memory>,

    #[serde(skip, default = "init_stable_user_stat")]
    pub user_stat: UserStatStable,
    #[serde(skip, default = "init_stable_user_stake_rank")]
    pub user_stake_rank: UserStakeRankStable,

    #[serde(skip, default = "init_stable_point_admin")]
    pub point_admin: PointAdminStable,
    #[serde(skip, default = "init_stable_staked_nft_reward")]
    pub staked_nft_reward: StakedNftRewardStable,
}

impl Default for State {
    fn default() -> Self {
        Self {
            metadata: init_stable_metadata(),
            // point_records: init_stable_point_record(),
            // next_period: init_stable_next_period(),
            btc_price: init_stable_btc_price(),
            reward_record: init_stable_reward_record(),
            // box_reward_record: init_stable_box_reward_record(),
            box_record: init_stable_box_record(),
            // user_status_record: init_stable_user_status_record(),
            box_reward: init_stable_box_reward(),
            invite_reward: init_stable_invite_reward(),
            stake_reward: init_stable_stake_reward(),
            config_setting: init_stable_config(),
            leader_board_status: init_leader_board_status(),
            user_rank: init_stable_user_rank(),
            timer_settings: init_stable_timer_settings(),
            user_stat: init_stable_user_stat(),
            user_stake_rank: init_stable_user_stake_rank(),
            point_admin: init_stable_point_admin(),
            staked_nft_reward: init_stable_staked_nft_reward(),
        }
    }
}

fn init_stable_metadata() -> StableCell<Metadata, Memory> {
    StableCell::init(memory::get_metadata_memory(), Metadata::default())
        .expect("failed to initialize the metadata cell")
}

// fn init_stable_point_record() -> PointRecordsStable {
//     StableBTreeMap::init(memory::get_point_records_memory())
// }

// fn init_stable_next_period() -> StableCell<u128, Memory> {
//     StableCell::init(memory::get_next_period_memory(), 0u128)
//         .expect("Could not initialize sig count memory")
// }

fn init_stable_btc_price() -> StableCell<PriceRecord, Memory> {
    StableCell::init(memory::get_btc_price_memory(), PriceRecord::default())
        .expect("Could not initialize sig count memory")
}

fn init_stable_reward_record() -> RewardRecordStable {
    StableBTreeMap::init(memory::get_reward_record_memory())
}

// fn init_stable_box_reward_record() -> BoxRewardDetailStable {
//     StableBTreeMap::init(memory::get_box_reward_detail_memory())
// }

fn init_stable_box_record() -> BoxRecordStable {
    StableBTreeMap::init(memory::get_box_record_memory())
}

// fn init_stable_user_status_record() -> UserStatusRecordStable {
//     StableBTreeMap::init(memory::get_user_status_record_memory())
// }

fn init_stable_config() -> StableCell<ConfigSetting, Memory> {
    StableCell::init(memory::get_config_memory(), ConfigSetting::default())
        .expect("failed to initialize the config cell")
}

fn init_stable_box_reward() -> BoxRewardStable {
    StableBTreeMap::init(memory::get_box_reward_memory())
}

fn init_stable_stake_reward() -> StakeRewardStable {
    StableBTreeMap::init(memory::get_invite_reward_memory())
}

fn init_stable_invite_reward() -> InviteRewardStable {
    StableBTreeMap::init(memory::get_stake_reward_memory())
}

fn init_leader_board_status() -> StableCell<LeaderBoardStatus, Memory> {
    StableCell::init(
        memory::get_learder_board_memory(),
        LeaderBoardStatus::default(),
    )
    .expect("failed to initialize the leaderboard cell")
}

fn init_stable_user_rank() -> UserRankStable {
    StableBTreeMap::init(memory::get_user_rank_memory())
}

fn init_stable_timer_settings() -> StableCell<TimerSettings, Memory> {
    StableCell::init(
        memory::get_timer_settings_memory(),
        TimerSettings::default(),
    )
    .expect("failed to initialize the timer settings cell")
}

fn init_stable_user_stat() -> UserStatStable {
    StableBTreeMap::init(memory::get_user_stat_memory())
}

fn init_stable_user_stake_rank() -> UserStakeRankStable {
    StableBTreeMap::init(memory::get_user_stake_rank_memory())
}

fn init_stable_point_admin() -> PointAdminStable {
    StableBTreeMap::init(memory::get_point_admin_memory())
}

fn init_stable_staked_nft_reward() -> StakedNftRewardStable {
    StableBTreeMap::init(memory::get_staked_nft_memory())
}
