use bitcoin::Amount;
use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::{
    bitcoin::{BitcoinNetwork, Satoshi},
    main::CanisterId,
};
use serde::{Deserialize, Serialize};
use wallet::{
    tx::{RecipientAmount, RecipientAmountVec},
    utils::str_to_bitcoin_address,
};

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

impl TransferRequest {
    pub fn validate_address(
        &self,
        network: BitcoinNetwork,
    ) -> Result<RecipientAmountVec, WalletError> {
        let res: Result<Vec<RecipientAmount>, WalletError> =
            self.iter().map(|t| t.validate_address(network)).collect();
        res.map(|r| RecipientAmountVec { txs: r })
    }
}

impl TransferInfo {
    pub fn validate_address(
        &self,
        network: BitcoinNetwork,
    ) -> Result<RecipientAmount, WalletError> {
        let recipient = str_to_bitcoin_address(&self.recipient, network).map_err(|e| e.into());
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

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct RegisterStakingRequest {
    pub txid: String,
    pub sender: Principal,
    pub sender_address: String,
    pub sent_amount: Satoshi,
    pub sent_time: u64,
    pub network: BitcoinNetwork,
    pub staking_canister: CanisterId,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct TotalStakingRequest {
    pub sender_address: String,
    pub staking_canister: CanisterId,
}
