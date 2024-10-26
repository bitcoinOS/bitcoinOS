use bitcoin::{hashes::Hash, Txid};
use candid::CandidType;
use ic_cdk::api::management_canister::bitcoin::{BlockHash, GetUtxosResponse, Satoshi};
use serde::{Deserialize, Serialize};

use super::staking::{FundManagement, PoolStatus};

/// Response type of [bitcoin_get_utxos](super::bitcoin_get_utxos). Translate the Utxo `txid` to string
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub struct UtxosResponse {
    /// List of UTXOs.
    pub utxos: Vec<Utxo>,
    /// Hash of the tip block.
    pub tip_block_hash: BlockHash,
    /// Height of the tip height.
    pub tip_height: u32,
    /// Page reference when the response needs to be paginated.
    ///
    /// To be used in [UtxoFilter::Page].
    pub next_page: Option<Vec<u8>>,
}

/// Unspent transaction output (UTXO).
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub struct Utxo {
    /// See [Outpoint].
    pub outpoint: Outpoint,
    /// Value in the units of satoshi.
    pub value: Satoshi,
    /// Height in the chain.
    pub height: u32,
}

/// Identifier of [Utxo].
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub struct Outpoint {
    /// Transaction Identifier.
    pub txid: String,
    /// A implicit index number.
    pub vout: u32,
}

impl From<GetUtxosResponse> for UtxosResponse {
    fn from(response: GetUtxosResponse) -> Self {
        Self {
            utxos: response
                .utxos
                .into_iter()
                .map(|u| Utxo {
                    outpoint: Outpoint {
                        txid: Txid::from_raw_hash(
                            Hash::from_slice(&u.outpoint.txid).expect("Failed to hash from txid"),
                        )
                        .to_string(),
                        vout: u.outpoint.vout,
                    },
                    value: u.value,
                    height: u.height,
                })
                .collect(),
            tip_block_hash: response.tip_block_hash,
            tip_height: response.tip_height,
            next_page: response.next_page,
        }
    }
}

/// Response result of send transaction to bitcoin network
/// * Fields:
/// * txid: txid of transaction when send success, and error is `None`
/// * error_msg: error message when send failed, and txid is `None`
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct FinalizeTransactionResponse {
    pub txid: Option<String>,
    pub error_msg: Option<String>,
}

/// Response for `UpdateStakingPoolInfoRequest`
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UpdateStakingPoolInfoResponse {
    pub name: String,
    pub description: String,
    // the annual interest rate of the staking pool will less than 10000, it will divide by 10000 for compute
    pub annual_interest_rate: u16,
    pub duration_in_day: u64,
    pub status: PoolStatus,
    pub start_time: u64,
    pub end_time: u64,
    pub fund_management: FundManagement,
}
