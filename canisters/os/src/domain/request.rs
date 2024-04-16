use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct InitArgument {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct InitWalletArgument {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
}
