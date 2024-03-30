use bitcoin::{Address, ScriptBuf};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Wallet {
    // The witness script of the 2-of-2 multisig wallet.
    pub witness_script: ScriptBuf,
    // The wallet address.
    pub address: Address,
    // The derivation path of the wallet, derived from the user's principal.
    pub derivation_path: Vec<Vec<u8>>,
}

#[derive(CandidType, Clone, Deserialize, Debug, Serialize)]
pub struct ECDSAPublicKey {
    pub canister_id: Option<Principal>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Deserialize, Debug, Serialize)]
pub struct ECDSAPublicKeyReply {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Clone, Deserialize, Debug, Serialize)]
pub struct EcdsaKeyId {
    pub curve: EcdsaCurve,
    pub name: String,
}

#[derive(CandidType, Clone, Deserialize, Debug, Serialize)]
pub enum EcdsaCurve {
    #[serde(rename = "secp256k1")]
    Secp256k1,
}
