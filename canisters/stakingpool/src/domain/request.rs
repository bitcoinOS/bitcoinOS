use bitcoin::Address;
use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, Satoshi};
use serde::Deserialize;
use wallet::utils::str_to_bitcoin_address;

use crate::error::StakingError;
use wallet::domain::staking::TxId;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct RegisterStakingRequest {
    pub txid: TxId,
    pub sender: Principal,
    pub sender_address: String,
    pub sent_amount: Satoshi,
    pub sent_time: u64,
    pub network: BitcoinNetwork,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct RedeemRequest {
    pub txid: TxId,
    pub recipient: String,
    pub network: BitcoinNetwork,
}

impl RedeemRequest {
    pub fn validate_address(&self) -> Result<Address, StakingError> {
        str_to_bitcoin_address(&self.recipient, self.network)
            .map_err(|e| StakingError::BitcoinAddressNetworkUnmatch(e.to_string()))
    }
}
