use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct InitPointArgument {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub os_canister: Principal,
    pub siwb_canister: Option<Principal>,
    pub task_period: u64,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct UserRequest {
    pub user_id: Principal,
    pub user_type: Principal,
    pub wallet_address: Option<String>,
    pub sign_msg: Option<String>,
    pub box_id: Option<u64>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct TimerSettingsRequest {
    pub save_point_in_secs: Option<u64>,
    pub create_staking_record_in_secs: Option<u64>,
    pub update_btc_price_in_secs: Option<u64>,
    pub update_leaderboard_in_secs: Option<u64>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct NftRecordRequest {
    pub user_id: Principal,
    pub nft_count: u64,
    pub nft_price: u64,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct StakeRecordRequest {
    pub user_id: Principal,
    pub sat: u64,
}