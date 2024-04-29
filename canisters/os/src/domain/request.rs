use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
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
}

#[derive(CandidType, Deserialize, Clone)]
pub struct InitStakingPoolArgument {
    pub name: String,
    pub description: String,
    pub annual_interest_rate: u64,
}
