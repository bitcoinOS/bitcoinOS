use std::cell::RefCell;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, RestrictedMemory,
};

pub type DefMem = DefaultMemoryImpl;
pub type RM = RestrictedMemory<DefMem>;
pub type VM = VirtualMemory<RM>;

pub type Memory = VirtualMemory<DefMem>;

// A memory for the StableBTreeMap we're using. A new memory should be created for
// every additional stable structure.
const METADATA_MEMORY_ID: MemoryId = MemoryId::new(1);
const STAKING_COUNTER_MEMORY_ID: MemoryId = MemoryId::new(2);
const REDEEM_COUNTER_MEMORY_ID: MemoryId = MemoryId::new(3);
const WALLET_MEMORY_ID: MemoryId = MemoryId::new(4);
const STAKING_RECORD_MEMORY_ID: MemoryId = MemoryId::new(5);
const REDEEM_LOG_IDX_MEM_ID: MemoryId = MemoryId::new(6);
const REDEEOM_LOG_DATA_MEM_ID: MemoryId = MemoryId::new(7);
const STAKER_MEMORY_ID: MemoryId = MemoryId::new(8);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
}

pub(super) fn get_metadata_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(METADATA_MEMORY_ID))
}

pub(super) fn get_staking_counter_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKING_COUNTER_MEMORY_ID))
}

pub(super) fn get_redeem_counter_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(REDEEM_COUNTER_MEMORY_ID))
}

pub(super) fn get_wallet_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(WALLET_MEMORY_ID))
}

pub(super) fn get_staking_record_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKING_RECORD_MEMORY_ID))
}

pub(super) fn get_staker_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKER_MEMORY_ID))
}

pub(super) fn get_redeem_log_index_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(REDEEM_LOG_IDX_MEM_ID))
}

pub(super) fn get_redeem_log_data_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(REDEEOM_LOG_DATA_MEM_ID))
}
