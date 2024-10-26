use candid::CandidType;
use ic_cdk::api::management_canister::main::CanisterId;
use serde::{Deserialize, Serialize};

// #[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
// pub struct RegisterStakingRequest {
//     pub txid: String,
//     pub sender: Principal,
//     pub sender_address: String,
//     pub sent_amount: Satoshi,
//     pub sent_time: u64,
//     pub network: BitcoinNetwork,
//     pub staking_canister: CanisterId,
// }

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct TotalStakingRequest {
    pub sender_address: String,
    pub staking_canister: CanisterId,
}
