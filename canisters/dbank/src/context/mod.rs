pub mod memory;

use std::cell::RefCell;

use crate::domain::{
    DBankWalletInfo, Metadata, RawWallet, SelfCustodyKey, TransactionLog, WalletOperationEvent,
};

use ic_stable_structures::{BTreeMap as StableBTreeMap, Cell as StableCell, Log as StableLog};
use serde::{Deserialize, Serialize};

use self::memory::Memory;

pub type Timestamp = u64;
pub type RawWalletStable = StableBTreeMap<SelfCustodyKey, RawWallet, Memory>;
pub type TransactionLogStable = StableLog<TransactionLog, Memory, Memory>;
pub type WalletInfoStable = StableBTreeMap<SelfCustodyKey, DBankWalletInfo, Memory>;
pub type WalletLogStable = StableLog<WalletOperationEvent, Memory, Memory>;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_metadata")]
    pub metadata: StableCell<Metadata, Memory>,
    #[serde(skip, default = "init_stable_sequencer")]
    pub sequencer: StableCell<u64, Memory>,
    #[serde(skip, default = "init_stable_wallet")]
    pub wallets: RawWalletStable,
    #[serde(skip, default = "init_stable_transaction_log")]
    pub tx_logs: TransactionLogStable,
    #[serde(skip, default = "init_stable_wallet_info")]
    pub wallet_infos: WalletInfoStable,
    #[serde(skip, default = "init_stable_wallet_log")]
    pub wallet_logs: WalletLogStable,
}

impl Default for State {
    fn default() -> Self {
        Self {
            metadata: init_stable_metadata(),
            sequencer: init_stable_sequencer(),
            wallets: init_stable_wallet(),
            tx_logs: init_stable_transaction_log(),
            wallet_infos: init_stable_wallet_info(),
            wallet_logs: init_stable_wallet_log(),
        }
    }
}

fn init_stable_metadata() -> StableCell<Metadata, Memory> {
    StableCell::init(memory::get_metadata_memory(), Metadata::default())
        .expect("failed to initialize the metadata cell")
}

fn init_stable_sequencer() -> StableCell<u64, Memory> {
    StableCell::init(memory::get_sequencer_memory(), 0u64)
        .expect("Could not initialize sequencer memory")
}

fn init_stable_wallet() -> RawWalletStable {
    StableBTreeMap::init(memory::get_wallet_memory())
}

fn init_stable_wallet_info() -> WalletInfoStable {
    StableBTreeMap::init(memory::get_wallet_info_memory())
}

fn init_stable_transaction_log() -> TransactionLogStable {
    StableLog::init(
        memory::get_transaction_log_index_memory(),
        memory::get_transaction_log_data_memory(),
    )
    .expect("failed to init transaction log")
}

fn init_stable_wallet_log() -> WalletLogStable {
    StableLog::init(
        memory::get_wallet_log_index_memory(),
        memory::get_wallet_log_data_memory(),
    )
    .expect("failed to init wallet log")
}
