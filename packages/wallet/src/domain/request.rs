use bitcoin::Amount;
use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::{
    bitcoin::{BitcoinNetwork, Satoshi, UtxoFilter},
    main::CanisterId,
};
use serde::{Deserialize, Serialize};

use crate::domain::staking::StakingType;
use crate::domain::TxId;
use crate::{
    error::Error,
    tx::{RawTransactionInfo, RecipientAmount, RecipientAmountVec},
    utils::str_to_bitcoin_address,
};

use super::staking::FundManagement;
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
    pub memo: Option<String>,
    pub fund_management: Option<String>,
    pub stake_type: Option<StakingType>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct RegisterStakingRecordRequest {
    pub txid: TxId,
    pub sender: Principal,
    pub sender_address: String,
    pub sent_amount: Satoshi,
    pub sent_time: u64,
    pub network: BitcoinNetwork,
    pub memo: Option<String>,
    pub stake_type: StakingType,
    pub fund_management: String,
    pub staking_canister: CanisterId,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UpdateStakingPoolInfoRequest {
    pub staking_pool_canister: CanisterId,
    pub name: String,
    pub description: String,
    // the annual interest rate of the staking pool will less than 10000, it will divide by 10000 for compute
    pub annual_interest_rate: u16,
    pub duration_in_day: u64,
    pub status: String,
    pub start_time: u64,
    pub end_time: u64,
    pub fund_management: FundManagement,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UpdateStakingPoolStatusRequest {
    pub staking_pool_canister: CanisterId,
    pub status: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CreateDBankWalletRequest {
    pub seq_in_os: u64,
    pub name: String,
    pub wallet_owner: Principal,
}
