pub mod nft_types;
pub mod request;
pub mod response;

use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

// use crate::constants::{DEFAULT_CYCLES_PER_CANISTER, DEFAULT_CYCLES_PER_DBANK};

// use ic_stable_structures::BTreeMap as StableBTreeMap;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Metadata {
    // pub nft_canister: Principal,
    pub os_canister: Principal,
    pub user_canister: Principal,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            // nft_canister: Principal::anonymous(),
            os_canister: Principal::anonymous(),
            user_canister: Principal::anonymous(),
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
pub struct StakeNFT {
    pub staker: Principal,
    pub nft_canister: Principal,
    pub nft_id: u32,
    pub from: Principal,
    pub to: Principal,
    pub amount: u8,
    pub stake_status: StakeStatus,
    pub stake_at: u64,
    pub unstake_at: u64,
}

/// For a type to be used in Stable storage like `StableBtreeMap`, it need to implement the `Storable` trait,
/// which specifies how the type can be serialized/deserialized.
impl Storable for StakeNFT {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StakeStatus {
    Stake,
    Unstake,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CandidType, Deserialize)]
pub struct StakeNFTKey {
    pub nft_canister: Principal,
    pub nft_id: u32,
}
impl Storable for StakeNFTKey {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}
