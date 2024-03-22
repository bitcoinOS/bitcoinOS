pub mod api;
pub mod constants;
pub mod domain;
pub mod error;
pub mod repoistories;
pub mod services;

use std::{borrow::Cow, cell::RefCell};

use candid::Principal;
use domain::{WalletAction, WalletOwner};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager as MM, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, Log as StableLog, RestrictedMemory, StableBTreeMap, Storable,
};

// const WASM_PAGE_SIZE: u64 = 65536;

// const GIB: usize = 1024 * 1024 * 1024;

// NOTE: we allocate the first 16 pages (about 2 MiB) of the
// canister memory for the metadata.
// const METADATA_PAGES: u64 = 16;

/// The maximum number of Wasm pages that we allow to use for the stable storage.
// const NUM_WASM_PAGES: u64 = 4 * (GIB as u64) / WASM_PAGE_SIZE;

// NOTE: ensure that all memory ids are unique and
// do not change across upgrades!
const WALLET_MEM_ID: MemoryId = MemoryId::new(0);
const WALLET_LOG_IDX_MEM_ID: MemoryId = MemoryId::new(1);
const WALLET_LOG_DATA_MEM_ID: MemoryId = MemoryId::new(2);

pub type DefMem = DefaultMemoryImpl;
pub type RM = RestrictedMemory<DefMem>;
pub type VM = VirtualMemory<RM>;

type Memory = VirtualMemory<DefMem>;

pub type WalletOwnerStable = StableBTreeMap<Principal, WalletOwner, Memory>;

pub type WalletActionStable = StableLog<WalletAction, Memory, Memory>;

thread_local! {

    // static METADATA: RefCell<StableCell<Cbor<Option<Metadata>>, RM>> =
    //     RefCell::new(StableCell::init(
    //         RM::new(DefMem::default(), 0..METADATA_PAGES),
    //         Cbor::default()
    //     ).expect("failed to initialized")
    //     );

    // static MEMORY_MANAGER: RefCell<MM<RM>> = RefCell::new(MM::init(RM::new(DefMem::default(), METADATA_PAGES..u64::MAX)));

    static MEMORY_MANAGER: RefCell<MM<DefaultMemoryImpl>> = RefCell::new(
        MM::init(DefaultMemoryImpl::default())
    );

    static WALLET_OWNER: RefCell<WalletOwnerStable> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(WALLET_MEM_ID))
        )
    );

    static WALLET_ACTION: RefCell<WalletActionStable> = RefCell::new(
        StableLog::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(WALLET_LOG_IDX_MEM_ID)),
            MEMORY_MANAGER.with(|m| m.borrow().get(WALLET_LOG_DATA_MEM_ID))
        ).expect("failed to init wallet log")
    )

}

/// A helper type implementing Storable for all
/// serde-serializable types using the CBOR encoding.
#[derive(Default)]
struct Cbor<T>(pub T)
where
    T: serde::Serialize + serde::de::DeserializeOwned;

impl<T> std::ops::Deref for Cbor<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Storable for Cbor<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        ciborium::ser::into_writer(&self.0, &mut buf).unwrap();
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(ciborium::de::from_reader(bytes.as_ref()).unwrap())
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 100,
        is_fixed_size: false,
    };
}
