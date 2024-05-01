pub mod request;
pub mod response;

use std::str::FromStr;

use bitcoin::{Address, ScriptBuf};
use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::{
    bitcoin::{BitcoinNetwork, Satoshi},
    ecdsa::EcdsaKeyId,
};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use wallet::domain::{AddressType, EcdsaKeyIds, Wallet, WalletType};

use crate::constants::{
    DAILY_LIMIET_SATOSHI, METADATA_SIZE, SELF_CUSTODY_SIZE, TRANSACTION_LOG_SIZE,
};

use self::request::TransferInfo;

pub type TxID = String;

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub owner: Principal,
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub ecdsa_key_id: EcdsaKeyId,
    pub daily_limit_satoshi: Satoshi,
    pub updated_time: u64,
}

impl Default for Metadata {
    fn default() -> Self {
        let network = BitcoinNetwork::Regtest;
        let ecdsa_key_id = EcdsaKeyIds::from(network).to_key_id();

        Self {
            name: "SmartWallet".to_string(),
            owner: Principal::anonymous(),
            steward_canister: Principal::anonymous(),
            network,
            ecdsa_key_id,
            daily_limit_satoshi: DAILY_LIMIET_SATOSHI,
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

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct RawWallet {
    pub witness_script: Vec<u8>,
    pub address: String,
    pub derivation_path: Vec<Vec<u8>>,
    pub wallet_type: WalletType,
}

impl From<RawWallet> for Wallet {
    fn from(wallet: RawWallet) -> Self {
        Self {
            witness_script: ScriptBuf::from_bytes(wallet.witness_script),
            address: Address::from_str(&wallet.address).unwrap().assume_checked(),
            derivation_path: wallet.derivation_path,
            wallet_type: wallet.wallet_type,
        }
    }
}

impl From<Wallet> for RawWallet {
    fn from(wallet: Wallet) -> Self {
        Self {
            witness_script: ScriptBuf::into_bytes(wallet.witness_script),
            address: wallet.address.to_string(),
            derivation_path: wallet.derivation_path,
            wallet_type: wallet.wallet_type,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelfCustodyKey {
    pub network: BitcoinNetwork,
    pub owner: Principal,
    pub steward_canister: Principal,
    pub wallet_type: WalletType,
    pub address_type: AddressType,
}

impl SelfCustodyKey {
    pub fn new(metadata: &Metadata, wallet_type: WalletType, address_type: AddressType) -> Self {
        Self {
            network: metadata.network,
            owner: metadata.owner,
            steward_canister: metadata.steward_canister,
            wallet_type,
            address_type,
        }
    }
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

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TransactionLog {
    pub txs: Vec<TransferInfo>,
    pub sender: Principal,
    pub send_time: u64,
}

impl Storable for TransactionLog {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: TRANSACTION_LOG_SIZE as u32,
        is_fixed_size: false,
    };
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TransactionLedger {
    pub txs: Vec<TransferInfo>,
    pub sender: Principal,
    pub send_time: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StakingRecord {
    pub txid: TxID,
    pub sender: Principal,
    pub sender_address: String,
    pub sent_amount: Satoshi,
    pub sent_time: u64,
    pub network: BitcoinNetwork,
}
