use base::ICBitcoinNetwork;
use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct NetworkResponse {
    network: String,
}

impl From<ICBitcoinNetwork> for NetworkResponse {
    fn from(network: ICBitcoinNetwork) -> Self {
        Self {
            network: format!("{:?}", network),
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PublicKeyResponse {
    pub public_key_hex: String,
}
