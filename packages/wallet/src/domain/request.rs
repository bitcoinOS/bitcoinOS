use bitcoin::Amount;
use candid::CandidType;
use ic_cdk::api::management_canister::{
    bitcoin::{BitcoinNetwork, Satoshi, UtxoFilter},
    main::CanisterId,
};
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    tx::{RawTransactionInfo, RecipientAmount, RecipientAmountVec},
    utils::str_to_bitcoin_address,
};

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

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct TransferRequest {
    pub txs: Vec<TransferInfo>,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct TransferInfo {
    pub recipient: String,
    pub amount: u64,
}

impl TransferRequest {
    pub fn iter(&self) -> impl Iterator<Item = &TransferInfo> {
        self.txs.iter()
    }
}

impl IntoIterator for TransferRequest {
    type Item = TransferInfo;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.txs.into_iter()
    }
}

impl TransferRequest {
    pub fn validate_address(&self, network: BitcoinNetwork) -> Result<RecipientAmountVec, Error> {
        let res: Result<Vec<RecipientAmount>, Error> =
            self.iter().map(|t| t.validate_address(network)).collect();
        res.map(|r| RecipientAmountVec { txs: r })
    }
}

impl TransferInfo {
    pub fn validate_address(&self, network: BitcoinNetwork) -> Result<RecipientAmount, Error> {
        let recipient = str_to_bitcoin_address(&self.recipient, network);
        recipient.map(|r| RecipientAmount {
            recipient: r,
            amount: Amount::from_sat(self.amount),
        })
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct StakingRequest {
    pub staking_canister: CanisterId,
    pub staking_address: String,
    pub amount: Satoshi,
}
