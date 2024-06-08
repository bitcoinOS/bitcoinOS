use candid::{CandidType, Principal};
use serde::Deserialize;
use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, main::CanisterId};

#[derive(Debug, CandidType, Deserialize)]
pub struct InitPointArgument {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub os_canister: Principal,
}