pub mod memory;

use std::{cell::RefCell, collections::BTreeMap};

use crate::domain::{Metadata, RawWallet, SelfCustodyKey, TransactionLog};

use ic_cdk_timers::TimerId;
use ic_stable_structures::{BTreeMap as StableBTreeMap, Cell as StableCell, Log as StableLog};
use serde::{Deserialize, Serialize};
use wallet::domain::staking::{StakingRecord, TxId};

use self::memory::Memory;

pub type Timestamp = u64;
pub type RawWalletStable = StableBTreeMap<SelfCustodyKey, RawWallet, Memory>;
pub type TransactionLogStable = StableLog<TransactionLog, Memory, Memory>;
pub type StakingRecordStable = StableBTreeMap<TxId, StakingRecord, Memory>;
pub type StakingTimerStable = StableBTreeMap<TxId, Timestamp, Memory>;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());

    pub static TIMER_IDS: RefCell<BTreeMap<TimerId, Timestamp>> = const { RefCell::new(BTreeMap::new()) };

}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_metadata")]
    pub metadata: StableCell<Metadata, Memory>,
    #[serde(skip, default = "init_stable_counter")]
    pub counter: StableCell<u128, Memory>,
    #[serde(skip, default = "init_stable_wallet")]
    pub wallets: RawWalletStable,
    #[serde(skip, default = "init_stable_transaction_log")]
    pub logs: TransactionLogStable,
    #[serde(skip, default = "init_stable_staking_record")]
    pub staking_records: StakingRecordStable,
    #[serde(skip, default = "init_stable_staking_timer")]
    pub stakingtimers: StakingTimerStable,
}

impl Default for State {
    fn default() -> Self {
        Self {
            metadata: init_stable_metadata(),
            counter: init_stable_counter(),
            wallets: init_stable_wallet(),
            logs: init_stable_transaction_log(),
            staking_records: init_stable_staking_record(),
            stakingtimers: init_stable_staking_timer(),
        }
    }
}

fn init_stable_metadata() -> StableCell<Metadata, Memory> {
    StableCell::init(memory::get_metadata_memory(), Metadata::default())
        .expect("failed to initialize the metadata cell")
}

fn init_stable_counter() -> StableCell<u128, Memory> {
    StableCell::init(memory::get_counter_memory(), 0u128)
        .expect("Could not initialize sig count memory")
}

fn init_stable_wallet() -> RawWalletStable {
    StableBTreeMap::init(memory::get_wallet_memory())
}

fn init_stable_staking_record() -> StakingRecordStable {
    StableBTreeMap::init(memory::get_staking_record_memory())
}

fn init_stable_staking_timer() -> StakingTimerStable {
    StableBTreeMap::init(memory::get_staking_timer_memory())
}

fn init_stable_transaction_log() -> TransactionLogStable {
    StableLog::init(
        memory::get_transaction_log_index_memory(),
        memory::get_transaction_log_data_memory(),
    )
    .expect("failed to init wallet ledger")
}
