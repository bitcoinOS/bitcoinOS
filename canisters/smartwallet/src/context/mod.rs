pub mod memory;

use std::cell::RefCell;

use crate::domain::{Metadata, RawWallet, SelfCustodyKey, TransactionLog};

use ic_stable_structures::{BTreeMap as StableBTreeMap, Cell as StableCell, Log as StableLog};
use serde::{Deserialize, Serialize};

use self::memory::Memory;

pub type Timestamp = u64;
pub type RawWalletStable = StableBTreeMap<SelfCustodyKey, RawWallet, Memory>;
pub type TransactionLedgerStable = StableLog<TransactionLog, Memory, Memory>;

// const METADATA_PAGES: u64 = 64;

// const CONTROLLER_ID: MemoryId = MemoryId::new(2);

thread_local! {

    // static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
    //     MemoryManager::init(DefaultMemoryImpl::default())
    // );

    // pub static METADATA: RefCell<StableCell<Metadata, RM>> = RefCell::new(StableCell::init(
    //     RM::new(DefMem::default(), 0..METADATA_PAGES),
    //     Metadata::default(),
    //   ).expect("failed to initialize the metadata cell"));

    // // A Wallet canister can store a wallet with the same derivation path, wallet type, address type once.
    // pub static RAW_WALLET: RefCell<RawWalletStable> = RefCell::new(
    //     StableBTreeMap::init(
    //         MEMORY_MANAGER.with(|m| m.borrow().get(SELF_CUSTODY_MEMORY_ID))
    //     )
    // );

    // pub static TRANSACTION_LEDGER: RefCell<TransactionLedgerStable> = RefCell::new(
    //     StableLog::init(
    //         MEMORY_MANAGER.with(|m| m.borrow().get(TRANSACTION_LOG_IDX_MEM_ID)),
    //         MEMORY_MANAGER.with(|m| m.borrow().get(TRANSACTION_LOG_DATA_MEM_ID))
    //     ).expect("failed to init wallet log")
    // )
    pub static STATE: RefCell<State> = RefCell::new(State::default());

}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_metadata")]
    pub metadata: StableCell<Metadata, Memory>,
    #[serde(skip, default = "init_stable_wallet")]
    pub wallets: RawWalletStable,
    #[serde(skip, default = "init_stable_transaction_ledger")]
    pub transaction_ledger: TransactionLedgerStable,
}

impl Default for State {
    fn default() -> Self {
        Self {
            metadata: init_stable_metadata(),
            wallets: init_stable_wallet(),
            transaction_ledger: init_stable_transaction_ledger(),
        }
    }
}

fn init_stable_metadata() -> StableCell<Metadata, Memory> {
    StableCell::init(memory::get_metadata_memory(), Metadata::default())
        .expect("failed to initialize the metadata cell")
}

fn init_stable_wallet() -> RawWalletStable {
    StableBTreeMap::init(memory::get_wallet_memory())
}

fn init_stable_transaction_ledger() -> TransactionLedgerStable {
    StableLog::init(
        memory::get_transaction_ledger_index_memory(),
        memory::get_transaction_ledger_data_memory(),
    )
    .expect("failed to init wallet ledger")
}
