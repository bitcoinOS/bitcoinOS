pub mod memory;

use std::cell::RefCell;

use crate::domain::{Metadata, PointRecord};

use candid::Principal;
use ic_stable_structures::{BTreeMap as StableBTreeMap, Cell as StableCell};
use serde::{Deserialize, Serialize};

use self::memory::Memory;

pub type Timestamp = u64;

/// A WalletInfo stable storage has a key with `User Principal` and `Wallet Canister`
pub type PointRecordsStable = StableBTreeMap<Principal, PointRecord, Memory>;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_metadata")]
    pub metadata: StableCell<Metadata, Memory>,
    #[serde(skip, default = "init_stable_point_record")]
    pub point_records: PointRecordsStable,
    #[serde(skip, default = "init_stable_next_period")]
    pub next_period: StableCell<u128, Memory>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            metadata: init_stable_metadata(),
            point_records: init_stable_point_record(),
            next_period: init_stable_next_period(),
        }
    }
}

fn init_stable_metadata() -> StableCell<Metadata, Memory> {
    StableCell::init(memory::get_metadata_memory(), Metadata::default())
        .expect("failed to initialize the metadata cell")
}

fn init_stable_point_record() -> PointRecordsStable {
    StableBTreeMap::init(memory::get_point_records_memory())
}

fn init_stable_next_period() -> StableCell<u128, Memory> {
    StableCell::init(memory::get_next_period_memory(), 0u128)
        .expect("Could not initialize sig count memory")
}
