use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PublicKeyResponse {
    pub public_key_hex: String,
}
