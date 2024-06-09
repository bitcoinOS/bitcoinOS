use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct InitPointArgument {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub os_canister: Principal,
    pub task_period: u64,
}
