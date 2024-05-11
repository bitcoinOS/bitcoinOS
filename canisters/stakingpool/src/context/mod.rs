pub mod memory;

use std::cell::RefCell;

use crate::domain::{Metadata, RawWallet, RedeemLog, SelfCustodyKey};

use ic_cdk::api::management_canister::main::CanisterId;
use ic_stable_structures::{BTreeMap as StableBTreeMap, Cell as StableCell, Log as StableLog};
use serde::{Deserialize, Serialize};

use wallet::domain::staking::{StakingRecord, TxId};

use self::memory::Memory;

pub type Timestamp = u64;
pub type RawWalletStable = StableBTreeMap<SelfCustodyKey, RawWallet, Memory>;
pub type RedeemLogStable = StableLog<RedeemLog, Memory, Memory>;
pub type StakingRecordStable = StableBTreeMap<TxId, StakingRecord, Memory>;
pub type StakerStable = StableBTreeMap<CanisterId, Timestamp, Memory>;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());

}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_metadata")]
    pub metadata: StableCell<Metadata, Memory>,
    #[serde(skip, default = "init_stable_staking_counter")]
    pub staking_counter: StableCell<u128, Memory>,
    #[serde(skip, default = "init_stable_redeem_counter")]
    pub redeem_counter: StableCell<u128, Memory>,
    #[serde(skip, default = "init_stable_wallet")]
    pub wallets: RawWalletStable,
    #[serde(skip, default = "init_stable_staking_record")]
    pub staking_records: StakingRecordStable,
    #[serde(skip, default = "init_stable_redeem_log")]
    pub redeem_logs: RedeemLogStable,
    #[serde(skip, default = "init_stable_staker")]
    pub stakers: StakerStable,
}

impl Default for State {
    fn default() -> Self {
        Self {
            metadata: init_stable_metadata(),
            staking_counter: init_stable_staking_counter(),
            redeem_counter: init_stable_redeem_counter(),
            wallets: init_stable_wallet(),
            staking_records: init_stable_staking_record(),
            redeem_logs: init_stable_redeem_log(),
            stakers: init_stable_staker(),
        }
    }
}

fn init_stable_metadata() -> StableCell<Metadata, Memory> {
    StableCell::init(memory::get_metadata_memory(), Metadata::default())
        .expect("failed to initialize the metadata cell")
}

fn init_stable_staking_counter() -> StableCell<u128, Memory> {
    StableCell::init(memory::get_staking_counter_memory(), 0u128)
        .expect("Could not initialize sig count memory")
}

fn init_stable_redeem_counter() -> StableCell<u128, Memory> {
    StableCell::init(memory::get_redeem_counter_memory(), 0u128)
        .expect("Could not initialize sig count memory")
}

fn init_stable_wallet() -> RawWalletStable {
    StableBTreeMap::init(memory::get_wallet_memory())
}

fn init_stable_staking_record() -> StakingRecordStable {
    StableBTreeMap::init(memory::get_staking_record_memory())
}

fn init_stable_staker() -> StakerStable {
    StableBTreeMap::init(memory::get_staker_memory())
}

fn init_stable_redeem_log() -> RedeemLogStable {
    StableLog::init(
        memory::get_redeem_log_index_memory(),
        memory::get_redeem_log_data_memory(),
    )
    .expect("failed to init wallet ledger")
}
