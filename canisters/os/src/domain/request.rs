use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, main::CanisterId};
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct InitArgument {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct InitWalletArgument {
    pub name: String,
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub owner: Option<Principal>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CreateStakingPoolRequest {
    pub name: String,
    pub description: String,
    pub annual_interest_rate: u64,
    pub duration_in_millisecond: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct InitStakingPoolArgument {
    pub name: String,
    pub description: String,
    pub network: BitcoinNetwork,
    pub annual_interest_rate: u64,
    pub duration_in_millisecond: u64,
    pub os_canister: CanisterId,
}
