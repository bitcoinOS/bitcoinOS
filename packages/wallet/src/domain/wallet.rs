use std::str::FromStr;

use bitcoin::{Address, ScriptBuf};
use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, main::CanisterId};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

use super::{AddressType, Wallet, WalletType};

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

    pub fn key_by_p2wpkh_single(
        network: BitcoinNetwork,
        wallet_owner: Principal,
        steward_canister: CanisterId,
    ) -> Self {
        Self::new(
            network,
            wallet_owner,
            steward_canister,
            WalletType::Single,
            AddressType::P2wpkh,
        )
    }

    pub fn key_by_p2pkh_single(
        network: BitcoinNetwork,
        wallet_owner: Principal,
        steward_canister: CanisterId,
    ) -> Self {
        Self::new(
            network,
            wallet_owner,
            steward_canister,
            WalletType::Single,
            AddressType::P2pkh,
        )
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
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct DBankWalletInfo {
    pub seq_in_os: u64,   // the sequence number of wallet in os
    pub seq_in_bank: u64, // he sequence number of wallet in bank
    pub name: String,
    pub owner: Principal,
    pub bitcoin_address: String,
    pub public_key: Vec<u8>,
    pub network: BitcoinNetwork,
    pub address_type: AddressType,
    pub wallet_type: WalletType,
    pub dbank_canister: CanisterId,
    pub steward_canister: CanisterId,
    pub status: DBankWalletStatus,
    pub created_at: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, Default)]
pub enum DBankWalletStatus {
    #[default]
    Activing,
    Inactive,
    Forbidden,
}

impl Storable for DBankWalletInfo {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}
