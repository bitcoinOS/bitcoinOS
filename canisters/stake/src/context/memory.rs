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
const STAKE_NFT_MEMORY_ID: MemoryId = MemoryId::new(2);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
}

pub(super) fn get_metadata_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(METADATA_MEMORY_ID))
}

pub(super) fn get_satke_nft_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKE_NFT_MEMORY_ID))
}
