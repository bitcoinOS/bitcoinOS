pub mod request;
pub mod response;

use std::str::FromStr;

use base::{domain::EcdsaKeyIds, ICBitcoinNetwork};
use bitcoin::{Address, ScriptBuf};
use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

use crate::constants::{METADATA_SIZE, SELF_CUSTODY_SIZE};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Metadata {
    pub owner: Principal,
    pub network: ICBitcoinNetwork,
    pub steward_canister: Principal,
    pub ecdsa_key_id: EcdsaKeyId,
    pub updated_time: u64,
}

impl Default for Metadata {
    fn default() -> Self {
        let network = ICBitcoinNetwork::Regtest;
        let ecdsa_key_id = EcdsaKeyIds::from(network).to_key_id();

        Self {
            owner: Principal::anonymous(),
            steward_canister: Principal::anonymous(),
            network,
            ecdsa_key_id,
            updated_time: 0,
        }
    }
}

impl Storable for Metadata {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: METADATA_SIZE as u32,
        is_fixed_size: false,
    };
}

pub type Wallet = base::domain::Wallet;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct RawWallet {
    pub witness_script: Vec<u8>,
    pub address: String,
    pub derivation_path: Vec<Vec<u8>>,
}

impl From<RawWallet> for Wallet {
    fn from(wallet: RawWallet) -> Self {
        Self {
            witness_script: ScriptBuf::from_bytes(wallet.witness_script),
            address: Address::from_str(&wallet.address).unwrap().assume_checked(),
            derivation_path: wallet.derivation_path,
        }
    }
}

impl From<Wallet> for RawWallet {
    fn from(wallet: Wallet) -> Self {
        Self {
            witness_script: ScriptBuf::into_bytes(wallet.witness_script),
            address: wallet.address.to_string(),
            derivation_path: wallet.derivation_path,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelfCustodyKey {
    pub network: ICBitcoinNetwork,
    pub owner: Principal,
    pub steward_canister: Principal,
}

impl Storable for SelfCustodyKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: SELF_CUSTODY_SIZE as u32,
        is_fixed_size: false,
    };
}

impl Storable for RawWallet {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: SELF_CUSTODY_SIZE as u32,
        is_fixed_size: false,
    };
}
