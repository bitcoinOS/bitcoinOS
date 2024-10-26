use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct StakeRequest {
    pub nft_canister: Principal,
    pub nft_id: u32,
    pub nft_owner: Principal,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UnStakeRequest {
    pub nft_canister: Principal,
    pub nft_id: u32,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct InitArgument {
    pub os_canister: Principal,
    pub user_canister: Principal,
}
