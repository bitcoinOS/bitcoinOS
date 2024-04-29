pub mod memory;

use std::cell::RefCell;

use crate::domain::{Metadata, StakingPoolInfo, WalletAction, WalletOwner};

use ic_cdk::api::management_canister::main::CanisterId;
use ic_stable_structures::{BTreeMap as StableBTreeMap, Cell as StableCell, Log as StableLog};
use serde::{Deserialize, Serialize};

use self::memory::Memory;

pub type Timestamp = u64;
pub type WalletOwnerStable = StableBTreeMap<CanisterId, WalletOwner, Memory>;
pub type WalletLogStable = StableLog<WalletAction, Memory, Memory>;
pub type StakingPoolStable = StableBTreeMap<CanisterId, StakingPoolInfo, Memory>;

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
    #[serde(skip, default = "init_stable_wallet")]
    pub wallets: WalletOwnerStable,
    #[serde(skip, default = "init_stable_action_log")]
    pub logs: WalletLogStable,
    #[serde(skip, default = "init_stable_staking_pool")]
    pub staking_pools: StakingPoolStable,
}

impl Default for State {
    fn default() -> Self {
        Self {
            metadata: init_stable_metadata(),
            wallet_counter: init_stable_wallet_counter(),
            staking_pool_counter: init_stable_staking_pool_counter(),
            wallets: init_stable_wallet(),
            logs: init_stable_action_log(),
            staking_pools: init_stable_staking_pool(),
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

fn init_stable_wallet() -> WalletOwnerStable {
    StableBTreeMap::init(memory::get_wallet_memory())
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
