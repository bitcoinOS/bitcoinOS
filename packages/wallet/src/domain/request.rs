use candid::CandidType;
use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, UtxoFilter};
use serde::{Deserialize, Serialize};

use crate::tx::RawTransactionInfo;

/// Request argument for  `utxos` api
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub struct UtxosRequest {
    pub address: String,
    pub filter: Option<UtxoFilter>,
}

/// Request argument for `finalize_tx_and_send` api
#[derive(Debug, CandidType, Deserialize)]
pub struct FinalizeRequest {
    pub network: BitcoinNetwork,
    pub tx_info_bytes: RawTransactionInfo,
}
