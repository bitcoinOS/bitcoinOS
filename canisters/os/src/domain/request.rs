use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct CreateWalletRequest {
    // pub network: String,
    // pub steward_canister: String,
    pub key_name: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct InitArgument {
    pub network: String,
    pub steward_canister: String,
}

#[derive(CandidType, Deserialize)]
pub struct InitWalletArgument {
    pub network: String,
    pub steward_canister: String,
    pub key_name: String,
}
