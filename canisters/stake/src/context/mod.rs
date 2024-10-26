pub mod memory;

use std::cell::RefCell;

use ic_stable_structures::{BTreeMap as StableBTreeMap, Cell as StableCell};
use serde::{Deserialize, Serialize};

use crate::domain::{Metadata, StakeNFT, StakeNFTKey};

use self::memory::Memory;

pub type Timestamp = u64;

pub type StakeNFTStable = StableBTreeMap<StakeNFTKey, StakeNFT, Memory>;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());

}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_metadata")]
    pub metadata: StableCell<Metadata, Memory>,

    #[serde(skip, default = "init_stable_stake_nft")]
    pub stake_nft: StakeNFTStable,
}

impl Default for State {
    fn default() -> Self {
        Self {
            metadata: init_stable_metadata(),
            stake_nft: init_stable_stake_nft(),
        }
    }
}

fn init_stable_metadata() -> StableCell<Metadata, Memory> {
    StableCell::init(memory::get_metadata_memory(), Metadata::default())
        .expect("failed to initialize the metadata cell")
}

fn init_stable_stake_nft() -> StakeNFTStable {
    StableBTreeMap::init(memory::get_satke_nft_memory())
}
