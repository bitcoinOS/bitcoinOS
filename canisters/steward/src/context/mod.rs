use std::cell::RefCell;

use crate::domain::{ECDSAKey, Metadata};
use candid::Principal;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap as StableBTreeMap, Cell as StableCell, DefaultMemoryImpl, RestrictedMemory,
};

pub type DefMem = DefaultMemoryImpl;
pub type RM = RestrictedMemory<DefMem>;
pub type VM = VirtualMemory<RM>;

pub type Memory = VirtualMemory<DefMem>;

/// A wallet canister will has its unique ecdsa key (wallet_canister_id, ecdsa_key)
pub type ECDSAKeyStable = StableBTreeMap<Principal, ECDSAKey, Memory>;

const METADATA_PAGES: u64 = 64;

const ECDSA_KEY_ID: MemoryId = MemoryId::new(1);

thread_local! {

    pub static METADATA: RefCell<StableCell<Metadata, RM>> =
    RefCell::new(StableCell::init(
        RM::new(DefMem::default(), 0..METADATA_PAGES),
        Metadata::default(),
      ).expect("failed to initialize the metadata cell")
    );

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static ECDSA_KEYS: RefCell<ECDSAKeyStable> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(ECDSA_KEY_ID))
        )
    );
}
