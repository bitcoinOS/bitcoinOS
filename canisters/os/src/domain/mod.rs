pub mod request;

use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, main::CanisterId};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Metadata {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            steward_canister: Principal::anonymous(),
            network: BitcoinNetwork::Regtest,
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

/// The `State` will store the canister info when a user create a wallet.
/// A wallet is also a canister, call `SmartWallet`
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct WalletOwner {
    pub canister_id: Principal,
    pub owner: Principal,
    pub created_at: u64,
}

/// For a type to be used in Stable storage like `StableBtreeMap`, it need to implement the `Storable` trait,
/// which specifies how the type can be serialized/deserialized.
impl Storable for WalletOwner {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CandidType, Deserialize)]
pub struct WalletInfoKey {
    pub owner: Principal,
    pub wallet_canister: CanisterId,
}

impl Storable for WalletInfoKey {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

/// The information of a wallet
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct WalletInfo {
    pub name: String,
    pub owner: Principal,
    pub wallet_canister: CanisterId,
    pub bitcoin_address: String,
    pub network: BitcoinNetwork,
    pub steward_canister: CanisterId,
    pub created_at: u64,
}

impl Storable for WalletInfo {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize)]
pub struct WalletAction {
    pub operator: Principal,
    pub action: Action,
    pub op_time: u64,
}

#[derive(CandidType, Deserialize)]
pub enum Action {
    Create,
    Delete,
}

impl Storable for WalletAction {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum WalletType {
    SelfCustody,
}

impl WalletType {
    pub fn list_wallet_types() -> Vec<String> {
        vec!["SelfCustody".to_string()]
    }
}

/// Staking pool info will be stored in stable storage
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct StakingPoolInfo {
    pub staking_pool_canister: CanisterId,
    pub bitcoin_address: String,
    pub name: String,
    pub description: String,
    pub network: BitcoinNetwork,
    pub annual_interest_rate: u64,
    pub duration_in_millisecond: u64,
    pub os_canister: CanisterId,
    pub created_at: u64,
}

/// For a type to be used in Stable storage like `StableBtreeMap`, it need to implement the `Storable` trait,
/// which specifies how the type can be serialized/deserialized.
impl Storable for StakingPoolInfo {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}
