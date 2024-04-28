use bitcoin::{Address, ScriptBuf};
use candid::CandidType;
use ic_cdk::api::management_canister::{
    bitcoin::BitcoinNetwork,
    ecdsa::{EcdsaCurve, EcdsaKeyId},
};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Wallet {
    // The witness script of the wallet.
    pub witness_script: ScriptBuf,
    // The wallet address.
    pub address: Address,
    // The derivation path of the wallet, derived from the user's principal.
    pub derivation_path: Vec<Vec<u8>>,
    // The wallet type.
    pub wallet_type: WalletType,
}

/// A wallet type of contains Single signature or 2-of-2 multisig.
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WalletType {
    Single,
    MultiSig22,
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddressType {
    /// Pay to pubkey hash.
    P2pkh,
    /// Pay to script hash.
    P2sh,
    /// Pay to witness pubkey hash.
    P2wpkh,
    /// Pay to witness script hash.
    P2wsh,
    /// Pay to taproot.
    P2tr,
}

impl TryFrom<bitcoin::AddressType> for AddressType {
    type Error = Error;

    fn try_from(address_type: bitcoin::AddressType) -> Result<Self, Self::Error> {
        match address_type {
            bitcoin::AddressType::P2pkh => Ok(Self::P2pkh),
            bitcoin::AddressType::P2sh => Ok(Self::P2sh),
            bitcoin::AddressType::P2wpkh => Ok(Self::P2wpkh),
            bitcoin::AddressType::P2wsh => Ok(Self::P2wsh),
            bitcoin::AddressType::P2tr => Ok(Self::P2tr),
            _ => Err(Error::InvalidBitcoinAddress(address_type.to_string())),
        }
    }
}

impl Default for WalletType {
    fn default() -> Self {
        Self::Single
    }
}

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
