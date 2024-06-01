pub mod memory;

use std::cell::RefCell;

use crate::domain::{
    Metadata,PointRecord,UserStakePool
};

use ic_cdk::api::management_canister::main::CanisterId;
use ic_stable_structures::{BTreeMap as StableBTreeMap, Cell as StableCell, Log as StableLog};
use serde::{Deserialize, Serialize};

use self::memory::Memory;

pub type Timestamp = u64;



/// A WalletInfo stable storage has a key with `User Principal` and `Wallet Canister`
pub type PointRecordsStable = StableBTreeMap<UserStakePool, PointRecord, Memory>;
 
thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());

}



#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_metadata")]
    pub metadata: StableCell<Metadata, Memory>,
    #[serde(skip, default = "init_stable_point_record")]
    pub point_records: PointRecordsStable
}

impl Default for State {
    fn default() -> Self {
        Self {
            metadata: init_stable_metadata(),
            total_point: init_stable_total_point(),
           
        }
    }
}

fn init_stable_metadata() -> StableCell<Metadata, Memory> {
    StableCell::init(memory::get_metadata_memory(), Metadata::default())
        .expect("failed to initialize the metadata cell")
}

 
fn init_stable_point_record() -> TotalPointStable {
    StableBTreeMap::init(memory::get_point_records_memory())
}

 
