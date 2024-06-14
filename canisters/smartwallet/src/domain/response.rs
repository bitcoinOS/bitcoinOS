use candid::CandidType;
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use serde::Deserialize;

use super::{RawWallet, SelfCustodyKey};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct NetworkResponse {
    network: String,
}

impl From<BitcoinNetwork> for NetworkResponse {
    fn from(network: BitcoinNetwork) -> Self {
        Self {
            network: format!("{:?}", network),
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PublicKeyResponse {
    pub public_key_hex: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct ListWalletResponse {
    pub key: SelfCustodyKey,
    pub wallet: RawWallet,
}
