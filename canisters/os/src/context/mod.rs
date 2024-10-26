pub mod memory;

use std::cell::RefCell;

use crate::domain::{
    CanisterModuleInfo, DBankInfo, Metadata, UserBindKey, UserInfo, WalletAction, WalletBindInfo,
    WalletInfo, WalletInfoKey, WalletOwner,
};

use candid::Principal;
use ic_cdk::api::management_canister::main::CanisterId;
use ic_stable_structures::{BTreeMap as StableBTreeMap, Cell as StableCell, Log as StableLog};
use serde::{Deserialize, Serialize};
use wallet::domain::{
    staking::{StakingPoolInfo, StakingRecord},
    CanisterName, TxId,
};

use self::memory::Memory;

pub type Timestamp = u64;

pub type WalletOwnerStable = StableBTreeMap<CanisterId, WalletOwner, Memory>;
/// A WalletInfo stable storage has a key with `User Principal` and `Wallet Canister`
pub type WalletInfoStable = StableBTreeMap<WalletInfoKey, WalletInfo, Memory>;
/// A DBankWalletInfo stable storage hash a key with user's Principal, and the value is WalletInfo
pub type DBankWalletInfoStable = StableBTreeMap<Principal, WalletInfo, Memory>;
/// A DBankWalletOwner stable storage hash a key with user's Principal, and the value is WalletInfo
pub type DBankInfoStable = StableBTreeMap<u64, DBankInfo, Memory>;

pub type WalletLogStable = StableLog<WalletAction, Memory, Memory>;
pub type StakingPoolStable = StableBTreeMap<CanisterId, StakingPoolInfo, Memory>;
pub type StakingRecordStable = StableBTreeMap<TxId, StakingRecord, Memory>;
pub type CanisterModuleStable = StableBTreeMap<CanisterName, CanisterModuleInfo, Memory>;

pub type UserInfoStable = StableBTreeMap<Principal, UserInfo, Memory>;

pub type UserInviteCodeStable = StableBTreeMap<String, Principal, Memory>;

//store  user bind wallet

pub type WalletBindStable = StableBTreeMap<UserBindKey, WalletBindInfo, Memory>;

pub type WalletMapBindStable = StableBTreeMap<String, UserBindKey, Memory>;

pub type UserBindCountStable = StableBTreeMap<Principal, u64, Memory>;

pub type UserImageLinkStable = StableBTreeMap<u128, String, Memory>;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());

}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_metadata")]
    pub metadata: StableCell<Metadata, Memory>,
    #[serde(skip, default = "init_stable_wallet_counter")]
    pub wallet_counter: StableCell<u128, Memory>,
    #[serde(skip, default = "init_stable_staking_pool_counter")]
    pub staking_pool_counter: StableCell<u128, Memory>,
    #[serde(skip, default = "init_stable_wallet_owner")]
    pub wallet_owners: WalletOwnerStable,
    #[serde(skip, default = "init_stable_action_log")]
    pub logs: WalletLogStable,
    #[serde(skip, default = "init_stable_staking_pool")]
    pub staking_pools: StakingPoolStable,
    #[serde(skip, default = "init_stable_wallet_info")]
    pub wallet_infos: WalletInfoStable,
    #[serde(skip, default = "init_stable_staking_record")]
    pub staking_records: StakingRecordStable,
    #[serde(skip, default = "init_canister_module")]
    pub canister_module: CanisterModuleStable,
    #[serde(skip, default = "init_user_info")]
    pub user_info: UserInfoStable,
    // #[serde(skip, default = "init_login_log")]
    // pub login_log: StableCell<LogBuffer, Memory>,
    #[serde(skip, default = "init_stable_user_counter")]
    pub user_counter: StableCell<u128, Memory>,
    #[serde(skip, default = "init_stable_user_invite_code")]
    pub user_invite_code: UserInviteCodeStable,

    #[serde(skip, default = "init_stable_dbank_wallet_info")]
    pub dbank_wallet_infos: DBankWalletInfoStable,

    #[serde(skip, default = "init_stable_dbank_info")]
    pub dbank_infos: DBankInfoStable,
    #[serde(skip, default = "init_stable_wallet_bind_info")]
    pub user_bind_wallet_info: WalletBindStable,
    #[serde(skip, default = "init_stable_wallet_map_bind_info")]
    pub wallet_map_bind: WalletMapBindStable,
    #[serde(skip, default = "init_stable_user_bind_count")]
    pub user_bind_wallet_count: UserBindCountStable,
    #[serde(skip, default = "init_stable_user_image_link")]
    pub user_image_link: UserImageLinkStable,
}

impl Default for State {
    fn default() -> Self {
        Self {
            metadata: init_stable_metadata(),
            wallet_counter: init_stable_wallet_counter(),
            staking_pool_counter: init_stable_staking_pool_counter(),
            wallet_owners: init_stable_wallet_owner(),
            logs: init_stable_action_log(),
            staking_pools: init_stable_staking_pool(),
            wallet_infos: init_stable_wallet_info(),
            staking_records: init_stable_staking_record(),
            canister_module: init_canister_module(),
            user_info: init_user_info(),
            // login_log: init_login_log(),
            user_counter: init_stable_user_counter(),
            user_invite_code: init_stable_user_invite_code(),
            dbank_wallet_infos: init_stable_dbank_wallet_info(),
            dbank_infos: init_stable_dbank_info(),
            user_bind_wallet_count: init_stable_user_bind_count(),
            wallet_map_bind: init_stable_wallet_map_bind_info(),
            user_bind_wallet_info: init_stable_wallet_bind_info(),
            user_image_link: init_stable_user_image_link(),
        }
    }
}

fn init_stable_metadata() -> StableCell<Metadata, Memory> {
    StableCell::init(memory::get_metadata_memory(), Metadata::default())
        .expect("failed to initialize the metadata cell")
}

fn init_stable_wallet_counter() -> StableCell<u128, Memory> {
    StableCell::init(memory::get_wallet_counter_memory(), 0u128)
        .expect("Could not initialize sig count memory")
}

fn init_stable_staking_pool_counter() -> StableCell<u128, Memory> {
    StableCell::init(memory::get_staking_pool_counter_memory(), 0u128)
        .expect("Could not initialize sig count memory")
}

fn init_stable_wallet_owner() -> WalletOwnerStable {
    StableBTreeMap::init(memory::get_wallet_owner_memory())
}

fn init_stable_action_log() -> WalletLogStable {
    StableLog::init(
        memory::get_action_log_index_memory(),
        memory::get_action_log_data_memory(),
    )
    .expect("failed to init wallet ledger")
}

fn init_stable_staking_pool() -> StakingPoolStable {
    StableBTreeMap::init(memory::get_staking_pool_memory())
}

fn init_stable_wallet_info() -> WalletInfoStable {
    StableBTreeMap::init(memory::get_wallet_info_memory())
}

fn init_stable_staking_record() -> StakingRecordStable {
    StableBTreeMap::init(memory::get_staking_record_memory())
}

fn init_canister_module() -> CanisterModuleStable {
    StableBTreeMap::init(memory::get_canister_module_memory())
}

fn init_user_info() -> UserInfoStable {
    StableBTreeMap::init(memory::get_user_info_memory())
}

// fn init_login_log() -> StableCell<LogBuffer, Memory> {
//     StableCell::init(memory::get_login_log_memory(), LogBuffer::default())
//         .expect("Could not initialize sig count memory")
// }

fn init_stable_user_counter() -> StableCell<u128, Memory> {
    StableCell::init(memory::get_user_counter_memory(), 0u128)
        .expect("Could not initialize sig count memory")
}
fn init_stable_user_invite_code() -> UserInviteCodeStable {
    StableBTreeMap::init(memory::get_user_invite_code_memory())
}

fn init_stable_dbank_wallet_info() -> DBankWalletInfoStable {
    StableBTreeMap::init(memory::get_dbank_wallet_info_memory())
}

fn init_stable_dbank_info() -> DBankInfoStable {
    StableBTreeMap::init(memory::get_dbank_info_memory())
}

fn init_stable_wallet_bind_info() -> WalletBindStable {
    StableBTreeMap::init(memory::get_wallet_bind_memory())
}

fn init_stable_wallet_map_bind_info() -> WalletMapBindStable {
    StableBTreeMap::init(memory::get_wallet_map_bind_memory())
}

fn init_stable_user_bind_count() -> UserBindCountStable {
    StableBTreeMap::init(memory::get_user_bind_count_memory())
}
fn init_stable_user_image_link() -> UserImageLinkStable {
    StableBTreeMap::init(memory::get_user_image_link_memory())
}
