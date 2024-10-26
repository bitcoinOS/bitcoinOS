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
    constants::{MAX_WALLET_LIMIT, START_ID_IN_DBANK_ID},
    domain::{request::TransferInfo, AddressType, EcdsaKeyIds, Wallet, WalletType},
    utils::mgmt_canister_id,
};

use crate::constants::DAILY_LIMIT_SATOSHI;

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Metadata {
    pub dbank_id: u64,
    pub name: String,
    pub owner: Principal,
    pub network: BitcoinNetwork,
    pub dbank_canister: CanisterId,
    pub steward_canister: Principal,
    pub ecdsa_key_id: EcdsaKeyId,
    pub daily_limit_satoshi: Satoshi,
    pub start_seq_in_os: u64,
    pub current_seq_in_os: u64,
    pub max_wallet_limit: u32,
    pub updated_time: u64,
}

impl Default for Metadata {
    fn default() -> Self {
        let network = BitcoinNetwork::Regtest;
        let ecdsa_key_id = EcdsaKeyIds::from(network).to_key_id();

        Self {
            dbank_id: START_ID_IN_DBANK_ID,
            name: format!("DBANK-{}", START_ID_IN_DBANK_ID),
            owner: Principal::anonymous(),
            dbank_canister: Principal::anonymous(),
            steward_canister: mgmt_canister_id(),
            network,
            ecdsa_key_id,
            daily_limit_satoshi: DAILY_LIMIT_SATOSHI,
            start_seq_in_os: 0,
            current_seq_in_os: 0,
            max_wallet_limit: MAX_WALLET_LIMIT,
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

#[derive(Clone, Copy, Debug, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelfCustodyKey {
    pub network: BitcoinNetwork,
    pub wallet_owner: Principal,
    pub steward_canister: CanisterId,
    pub wallet_type: WalletType,
    pub address_type: AddressType,
}

impl SelfCustodyKey {
    pub fn new(
        network: BitcoinNetwork,
        wallet_owner: Principal,
        steward_canister: CanisterId,
        wallet_type: WalletType,
        address_type: AddressType,
    ) -> Self {
        Self {
            network,
            wallet_owner,
            steward_canister,
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

/// The information of a wallet
pub type DBankWalletInfo = wallet::domain::wallet::DBankWalletInfo;
// #[derive(Debug, CandidType, Deserialize, Clone)]
// pub struct DBankWalletInfo {
//     pub seq_in_os: u64,   // the sequence number of wallet in os
//     pub seq_in_bank: u64, // he sequence number of wallet in bank
//     pub name: String,
//     pub owner: Principal,
//     pub bitcoin_address: String,
//     pub public_key: Vec<u8>,
//     pub network: BitcoinNetwork,
//     pub address_type: AddressType,
//     pub wallet_type: WalletType,
//     pub dbank_canister_id: CanisterId,
//     pub steward_canister: CanisterId,
//     pub status: DBankWalletStatus,
//     pub created_at: u64,
// }

// #[derive(CandidType, Deserialize, Debug, Clone, Default)]
// pub enum DBankWalletStatus {
//     #[default]
//     Activing,
//     Inactive,
//     Forbidden,
// }

// impl Storable for DBankWalletInfo {
//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }

//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         std::borrow::Cow::Owned(Encode!(self).unwrap())
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

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
pub enum WalletOperationEvent {
    CreateWallet(CreateWalletEvent),
    // DisableWallet(ForbiddenWalletLog),
    // PauseWallet(InactiveWalletLog),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct CreateWalletEvent {
    pub wallet_info: DBankWalletInfo,
}

impl Storable for WalletOperationEvent {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub enum EventLog {
//     TransactionLog(TransactionLog),
//     CreateWalletLog(CreateWalletLog),
// }

// impl Storable for EventLog {
//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         std::borrow::Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }
