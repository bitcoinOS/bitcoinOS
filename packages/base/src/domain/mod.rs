use bitcoin::{Address, ScriptBuf};
use candid::CandidType;
use ic_cdk::api::management_canister::{
    bitcoin::BitcoinNetwork,
    ecdsa::{EcdsaCurve, EcdsaKeyId},
};
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

// #[derive(CandidType, Clone, Deserialize, Debug, Serialize)]
// pub struct EcdsaPublicKeyRequest {
//     pub canister_id: Option<Principal>,
//     pub derivation_path: Vec<Vec<u8>>,
//     pub key_id: EcdsaKeyId,
// }

// #[derive(CandidType, Deserialize, Debug, Serialize)]
// pub struct EcdsaPublicKeyResponse {
//     pub public_key: Vec<u8>,
//     pub chain_code: Vec<u8>,
// }

// #[derive(CandidType, Clone, Deserialize, Debug, Serialize)]
// pub struct EcdsaKeyId {
//     pub curve: EcdsaCurve,
//     pub name: String,
// }

// #[derive(CandidType, Clone, Deserialize, Debug, Serialize)]
// pub enum EcdsaCurve {
//     #[serde(rename = "secp256k1")]
//     Secp256k1,
// }

pub enum EcdsaKeyIds {
    #[allow(unused)]
    TestKeyLocalDevelopment,
    #[allow(unused)]
    TestKey1,
    #[allow(unused)]
    ProductionKey1,
}

impl From<BitcoinNetwork> for EcdsaKeyIds {
    fn from(network: BitcoinNetwork) -> Self {
        match network {
            BitcoinNetwork::Mainnet => Self::ProductionKey1,
            BitcoinNetwork::Testnet => Self::TestKey1,
            BitcoinNetwork::Regtest => Self::TestKeyLocalDevelopment,
        }
    }
}

impl EcdsaKeyIds {
    pub fn to_key_id(&self) -> EcdsaKeyId {
        EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: match self {
                Self::TestKeyLocalDevelopment => "dfx_test_key",
                Self::TestKey1 => "test_key_1",
                Self::ProductionKey1 => "key_1",
            }
            .to_string(),
        }
    }
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SignWithEcdsaReply {
    pub signature: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug)]
pub struct SignWithEcdsa {
    pub message_hash: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

/// 2-2 MultiSignature a transaction will contains 2 signatures,
/// the first signature is the wallet sign, the second signature is the steward sign
/// the sequcence is [wallet_signature, steward_signature]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MultiSigIndex {
    First,  // For `Wallet` canister
    Second, // For `Steward` canister
}
