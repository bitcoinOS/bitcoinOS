pub mod request;
pub mod response;

use std::str::FromStr;

use bitcoin::{Address, ScriptBuf};
use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::{
    bitcoin::BitcoinNetwork, ecdsa::EcdsaKeyId, main::CanisterId,
};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use wallet::domain::{AddressType, EcdsaKeyIds, Wallet, WalletType};

use self::request::RedeemRequest;

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub network: BitcoinNetwork,
    // the annual interest rate of the staking pool will less than 10000, it will divide by 10000 for compute
    pub annual_interest_rate: u16,
    pub duration_in_day: u64,
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
            duration_in_day: 0,
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

// /// A Staking record is the record of a staked Bitcoin, its status will be `Pending` or `Confirmed` or `Redeeming` or `Redeemed`.
// /// When the record is created, it will be `Pending` and received_amount will be 0.
// /// When the staking transactoin is confirmed for 6 blocks by Bitcoin network, received_amount will be updated and status will be `Confirmed`.
// /// When the staking record is redeemed, its status will be `Redeeming`.
// /// When the redeemed tx is confirmed for 6 blocks by Bitcoin network, status will be `Redeemed`.
// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct StakingRecord {
//     pub txid: TxId,
//     pub sender: Principal,
//     pub sender_canister: CanisterId,
//     pub sender_address: String,
//     pub sent_amount: Satoshi,
//     pub sent_time: u64,
//     // the annual interest rate of the staking pool will less than 10000, it will divide by 10000 for compute
//     pub annual_interest_rate: u16,
//     pub duration_in_day: u64,
//     pub network: BitcoinNetwork,
//     pub staking_canister: CanisterId,
//     pub staking_address: String,
//     pub actual_amount: Satoshi,
//     pub status: StakingStatus,
//     pub updated_time: u64,
// }

// #[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
// pub enum StakingStatus {
//     Pending,
//     Confirmed,
//     Redeeming,
//     Redeemed,
// }

// impl Storable for StakingRecord {
//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         std::borrow::Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

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

    const BOUND: Bound = Bound::Unbounded;
}

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct TransactionLedger {
//     pub txs: Vec<RedeemRequest>,
//     pub sender: Principal,
//     pub send_time: u64,
// }
