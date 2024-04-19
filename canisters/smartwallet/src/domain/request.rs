use base::{
    tx::{TransactionInnerRequest, TransactionRequest},
    utils::str_to_bitcoin_address,
};
use candid::CandidType;
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use serde::Deserialize;

use crate::error::WalletError;

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

// impl TryFrom<TransferRequest> for TransactionRequest {
//     type Error = WalletError;

//     fn try_from(req: TransferRequest) -> Result<Self, Self::Error> {
//         let txs: Result<Vec<TransactionInnerRequest>, WalletError> = req
//             .into_iter()
//             .map(TransactionInnerRequest::try_from)
//             .collect();
//         txs.map(|t| TransactionRequest { txs: t })
//     }
// }

impl TransferRequest {
    pub fn validate_address(
        &self,
        network: BitcoinNetwork,
    ) -> Result<TransactionRequest, WalletError> {
        let res: Result<Vec<TransactionInnerRequest>, WalletError> =
            self.iter().map(|t| t.validate_address(network)).collect();
        res.map(|r| TransactionRequest { txs: r })
    }
}

impl TransferInfo {
    pub fn validate_address(
        &self,
        network: BitcoinNetwork,
    ) -> Result<TransactionInnerRequest, WalletError> {
        let recipient = str_to_bitcoin_address(&self.recipient, network).map_err(|e| e.into());
        recipient.map(|r| TransactionInnerRequest {
            recipient: r,
            amount: self.amount,
        })
    }
}
