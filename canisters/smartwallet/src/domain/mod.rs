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
use wallet::{
    domain::{AddressType, EcdsaKeyIds, Wallet, WalletType},
    utils::mgmt_canister_id,
};

use crate::constants::DAILY_LIMIET_SATOSHI;

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
            steward_canister: mgmt_canister_id(),
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

    const BOUND: Bound = Bound::Unbounded;
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

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for RawWallet {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
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

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TransactionLedger {
    pub txs: Vec<TransferInfo>,
    pub sender: Principal,
    pub send_time: u64,
}

/// A Staking record is the record of a staked Bitcoin, its status will be `Pending` or `Confirmed` or `Redeeming` or `Redeemed`.
/// When the record is created, it will be `Pending` and received_amount will be 0.
/// When the staking transactoin is confirmed for 6 blocks by Bitcoin network, received_amount will be updated and status will be `Confirmed`.
/// When the staking record is redeemed, its status will be `Redeeming`.
/// When the redeemed tx is confirmed for 6 blocks by Bitcoin network, status will be `Redeemed`.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StakingRecord {
    pub txid: TxID,
    pub sender: Principal,
    pub sender_canister: CanisterId,
    pub sender_address: String,
    pub sent_amount: Satoshi,
    pub sent_time: u64,
    pub duration_in_millisecond: u64,
    pub network: BitcoinNetwork,
    pub staking_canister: CanisterId,
    pub staking_address: String,
    pub actual_amount: Satoshi,
    pub status: StakingStatus,
    pub updated_time: u64,
}

impl StakingRecord {
    pub fn can_update(&self, other: &Self) -> bool {
        self.status.next() == Some(other.status)
            && self.txid == other.txid
            && self.sender == other.sender
            && self.sender_address == other.sender_address
            && self.sender_canister == other.sender_canister
            && self.sent_amount == other.sent_amount
            && self.actual_amount == other.actual_amount
            && self.network == other.network
            && self.updated_time < other.updated_time
    }
}

#[derive(Clone, Copy, Debug, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum StakingStatus {
    Pending,
    Confirmed,
    Redeeming,
    Redeemed,
}

impl StakingStatus {
    pub fn next(&self) -> Option<Self> {
        match self {
            StakingStatus::Pending => Some(StakingStatus::Confirmed),
            StakingStatus::Confirmed => Some(StakingStatus::Redeeming),
            StakingStatus::Redeeming => Some(StakingStatus::Redeemed),
            StakingStatus::Redeemed => None,
        }
    }
}

impl Storable for StakingRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
