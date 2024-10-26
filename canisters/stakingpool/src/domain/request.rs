use bitcoin::Address;
use candid::CandidType;
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use serde::Deserialize;
use wallet::utils::str_to_bitcoin_address;

use wallet::domain::TxId;
use wallet::error::StakingError;

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
