use candid::CandidType;
use ic_cdk::api::management_canister::bitcoin::UtxoFilter;
use serde::{Deserialize, Serialize};

/// Request argument for  `utxos` api
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub struct UtxosRequest {
    pub address: String,
    pub filter: Option<UtxoFilter>,
}
