pub mod request;

use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::{bitcoin::{BitcoinNetwork,Satoshi}, main::CanisterId};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

use crate::constants::{DEFAULT_TIME_PER_PERIOD,DEFAULT_POINT_PER_SAT,POINT_DECIMAL};

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Metadata {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub period: u8,
    pub point_per_sat: u8,
    pub point_decimal:u8,
    pub updated_time: u64,
}

impl Default for Metadata {
    fn default() -> Self {
        let network = BitcoinNetwork::Regtest;
        Self {
            network: network,
            steward_canister: Principal::anonymous(),
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
    pub stake_pool_canister: CanisterId,
    pub actual_amount:Satoshi,
    pub points:u64,
    pub updated_time:u64
}


impl PointRecord {
    pub fn can_update(&self, other: &Self) -> bool {
            self.staker == other.staker
            && self.stake_pool_canister == other.stake_pool_canister
            && self.network == other.network
            && self.updated_time < other.updated_time
    }
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UserStakePool{
   pub user: Principal,
   pub stake_pool: CanisterId
};

impl Storable for UserStakePool {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}