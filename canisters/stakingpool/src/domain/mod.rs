pub mod request;
pub mod response;

use std::str::FromStr;

use bitcoin::{Address, ScriptBuf};
use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::{
    bitcoin::{BitcoinNetwork, Satoshi},
    ecdsa::EcdsaKeyId,
    main::CanisterId,
};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use wallet::domain::{AddressType, EcdsaKeyIds, Wallet, WalletType};

use crate::constants::{METADATA_SIZE, REDEEM_LOG_SIZE, SELF_CUSTODY_SIZE, STAKING_RECORD_SIZE};

use self::request::RedeemRequest;

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub network: BitcoinNetwork,
    pub annual_interest_rate: u64,
    pub duration_in_month: u16,
    pub os_canister: CanisterId,
    pub ecdsa_key_id: EcdsaKeyId,
    pub updated_time: u64,
    pub owner: Principal,
}

impl Default for Metadata {
    fn default() -> Self {
        let network = BitcoinNetwork::Regtest;
        let ecdsa_key_id = EcdsaKeyIds::from(network).to_key_id();

        Self {
            name: "StakingPool".to_string(),
            description: "StakingPool".to_string(),
            network,
            annual_interest_rate: 0,
            duration_in_month: 0,
            os_canister: Principal::anonymous(),
            ecdsa_key_id,
            updated_time: 0,
            owner: Principal::anonymous(),
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
            steward_canister: metadata.os_canister,
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

/// A Staking record is the record of a staked Bitcoin, its status will be Pending or Confirmed.
/// When the record is created, it will be Pending.
/// When the staking transactoin is confirmed by Bitcoin network, it will be Confirmed.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StakingRecord {
    pub txid: String,
    pub sender: Principal,
    pub sender_address: String,
    pub amount: Satoshi,
    pub send_time: u64,
    pub status: StakingStatus,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum StakingStatus {
    Pending,
    Confirmed,
}

impl Storable for StakingRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: STAKING_RECORD_SIZE as u32,
        is_fixed_size: false,
    };
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct RedeemLog {
    pub req: RedeemRequest,
    pub sender: Principal,
    pub send_time: u64,
}

impl Storable for RedeemLog {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: REDEEM_LOG_SIZE as u32,
        is_fixed_size: false,
    };
}

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct TransactionLedger {
//     pub txs: Vec<RedeemRequest>,
//     pub sender: Principal,
//     pub send_time: u64,
// }
