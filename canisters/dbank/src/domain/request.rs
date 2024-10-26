use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CreateDBankWalletRequest {
    pub seq_in_os: u64,
    pub name: String,
    pub wallet_owner: Principal,
}
