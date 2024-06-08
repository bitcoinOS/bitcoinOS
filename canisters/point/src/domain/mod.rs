pub mod request;

use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::{bitcoin::{BitcoinNetwork,Satoshi}, main::CanisterId};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

use crate::constants::{DEFAULT_TIME_PER_PERIOD,DEFAULT_POINT_PER_SAT,POINT_DECIMAL};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Metadata {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub os_canister: Principal,
    pub period: u64,
    pub point_per_sat: u64,
    pub point_decimal:u64,
    pub updated_time: u64,
}

impl Default for Metadata {
    fn default() -> Self {
        let network = BitcoinNetwork::Regtest;
        Self {
            network: network,
            steward_canister: Principal::anonymous(),
            os_canister:Principal::anonymous(),
            period:DEFAULT_TIME_PER_PERIOD,
            point_per_sat:DEFAULT_POINT_PER_SAT,
            point_decimal:POINT_DECIMAL,
            updated_time:0
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
pub struct PointRecord {
    pub network: BitcoinNetwork,
    pub staker: Principal,
    // pub stake_pool_canister: CanisterId,
    pub actual_amount:Satoshi,
    pub points:u64,
    pub updated_time:u64
}


impl PointRecord {
    pub fn can_update(&self, other: &Self) -> bool {
            self.staker == other.staker
            // && self.stake_pool_canister == other.stake_pool_canister
            && self.network == other.network
            && self.updated_time < other.updated_time
    }
}


impl Storable for PointRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

// #[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
// pub struct UserStakePool{
//    pub user: Principal,
//    pub stake_pool: CanisterId
// }

// impl Storable for UserStakePool {
//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         std::borrow::Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

/// Staking pool info will be stored in stable storage
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct StakingPoolInfo {
    pub staking_pool_canister: CanisterId,
    pub bitcoin_address: String,
    pub name: String,
    pub description: String,
    pub network: BitcoinNetwork,
    pub annual_interest_rate: u16,
    pub duration_in_day: u64,
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