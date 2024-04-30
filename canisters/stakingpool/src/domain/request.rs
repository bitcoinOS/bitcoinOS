use bitcoin::Amount;
use candid::CandidType;
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use serde::Deserialize;
use wallet::{tx::RecipientAmount, utils::str_to_bitcoin_address};

use crate::error::WalletError;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct RedeemRequest {
    pub recipient: String,
    pub amount: u64,
}

impl RedeemRequest {
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
