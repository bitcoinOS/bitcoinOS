use std::cell::RefCell;

use crate::domain::{Metadata, RawWallet, SelfCustodyKey, TransactionLog};

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap as StableBTreeMap, Cell as StableCell, DefaultMemoryImpl, Log as StableLog,
    RestrictedMemory,
};

pub type Timestamp = u64;
pub type DefMem = DefaultMemoryImpl;
pub type RM = RestrictedMemory<DefMem>;
pub type VM = VirtualMemory<RM>;

pub type Memory = VirtualMemory<DefMem>;

pub type RawWalletStable = StableBTreeMap<SelfCustodyKey, RawWallet, Memory>;
pub type TransactionLogStable = StableLog<TransactionLog, Memory, Memory>;

const METADATA_PAGES: u64 = 64;

const SELF_CUSTODY_MEMORY_ID: MemoryId = MemoryId::new(1);
const TRANSACTION_LOG_IDX_MEM_ID: MemoryId = MemoryId::new(2);
const TRANSACTION_LOG_DATA_MEM_ID: MemoryId = MemoryId::new(3);
// const CONTROLLER_ID: MemoryId = MemoryId::new(2);

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static METADATA: RefCell<StableCell<Metadata, RM>> = RefCell::new(StableCell::init(
        RM::new(DefMem::default(), 0..METADATA_PAGES),
        Metadata::default(),
      ).expect("failed to initialize the metadata cell"));

    // A Wallet canister can store a wallet with the same derivation path, wallet type, address type once.
    pub static RAW_WALLET: RefCell<RawWalletStable> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(SELF_CUSTODY_MEMORY_ID))
        )
    );

    pub static TRANSACTION_LOG: RefCell<TransactionLogStable> = RefCell::new(
        StableLog::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(TRANSACTION_LOG_IDX_MEM_ID)),
            MEMORY_MANAGER.with(|m| m.borrow().get(TRANSACTION_LOG_DATA_MEM_ID))
        ).expect("failed to init wallet log")
    )
}
