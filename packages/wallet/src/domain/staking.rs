use std::{borrow::Cow, fmt::Display};

use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::{
    bitcoin::{BitcoinNetwork, Satoshi},
    main::CanisterId,
};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

use super::TxId;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct InitStakingPoolArgument {
    pub name: String,
    pub description: String,
    pub network: BitcoinNetwork,
    // the annual interest rate of the staking pool will less than 10000, it will divide by 10000 for compute
    pub annual_interest_rate: u16,
    pub duration_in_day: u64,
    pub os_canister: CanisterId,
    pub steward_canister: CanisterId,
    pub status: String,
    pub start_time: u64,
    // pub stake_end_time: u64,
    pub end_time: u64,
    pub fund_management: String,
    pub minimum_stake_amount: Option<u64>,
    pub boost_rate: Option<u64>,
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum FundManagement {
    Transfer,
    Locktime,
}

impl Default for FundManagement {
    fn default() -> Self {
        Self::Transfer
    }
}

impl From<String> for FundManagement {
    fn from(value: String) -> Self {
        match value.trim().to_lowercase().as_str() {
            "transfer" => Self::Transfer,
            "locktime" => Self::Locktime,
            _ => Self::default(),
        }
    }
}

impl Display for FundManagement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = format!("{:?}", self);
        write!(f, "{}", v.to_lowercase())
    }
}

#[derive(Clone, Copy, Debug, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum StakingType {
    BTCWallet,
    OSWallet,
}

impl Default for StakingType {
    fn default() -> Self {
        Self::OSWallet
    }
}

/// A Staking record is the record of a staked Bitcoin, its status will be `Pending` or `Confirmed` or `Redeeming` or `Redeemed`.
/// When the record is created, it will be `Pending` and received_amount will be 0.
/// When the staking transactoin is confirmed for 6 blocks by Bitcoin network, received_amount will be updated and status will be `Confirmed`.
/// When the staking record is redeemed, its status will be `Redeeming`.
/// When the redeemed tx is confirmed for 6 blocks by Bitcoin network, status will be `Redeemed`.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StakingRecord {
    pub txid: TxId,
    pub sender: Principal,
    pub sender_canister: CanisterId,
    pub sender_address: String,
    pub sent_amount: Satoshi,
    pub sent_time: u64,
    // the annual interest rate of the staking pool will less than 10000, it will divide by 10000 for compute, e.g 5.00% is 500
    pub annual_interest_rate: u16,
    pub duration_in_day: u64,
    pub network: BitcoinNetwork,
    pub staking_canister: CanisterId,
    pub staking_address: String,
    pub actual_amount: Satoshi,
    pub status: StakingStatus,
    pub redeemed_txid: Option<TxId>,
    pub updated_time: u64,
    pub memo: Option<String>,
    pub stake_type: StakingType,
    pub fund_management: FundManagement,
}

impl StakingRecord {
    pub fn can_update(&self, other: &Self) -> bool {
        // self.status.next() == Some(other.status)
        other.status.after(&self.status)
            && self.txid == other.txid
            && self.sender == other.sender
            && self.sender_address == other.sender_address
            && self.sender_canister == other.sender_canister
            && self.sent_amount == other.sent_amount
            && self.sent_amount >= other.actual_amount
            && self.network == other.network
            && self.updated_time < other.updated_time
    }

    pub fn is_tvl(&self) -> bool {
        self.status == StakingStatus::Pending || self.status == StakingStatus::Confirmed
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
    pub fn after(&self, other: &Self) -> bool {
        match (other, self) {
            (_, StakingStatus::Pending) => false,
            (StakingStatus::Pending, _) => true,
            (_, StakingStatus::Confirmed) => false,
            (StakingStatus::Confirmed, _) => true,
            (_, StakingStatus::Redeeming) => false,
            (StakingStatus::Redeeming, _) => true,
            (_, StakingStatus::Redeemed) => false,
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
    pub steward_canister: CanisterId,
    pub created_at: u64,
    pub status: PoolStatus,
    pub start_time: u64,
    // pub stake_end_time: u64,
    pub end_time: u64,
    pub fund_management: FundManagement,
    pub minimum_stake_amount: Option<u64>,
    pub boost_rate: Option<u64>,
}

impl From<StakingPoolInfo> for InitStakingPoolArgument {
    fn from(info: StakingPoolInfo) -> Self {
        Self {
            name: info.name,
            description: info.description,
            network: info.network,
            annual_interest_rate: info.annual_interest_rate,
            duration_in_day: info.duration_in_day,
            os_canister: info.os_canister,
            steward_canister: info.steward_canister,
            status: info.status.to_string(),
            start_time: info.start_time,
            // stake_end_time: info.stake_end_time,
            end_time: info.end_time,
            fund_management: info.fund_management.to_string(),
            minimum_stake_amount: info.minimum_stake_amount,
            boost_rate: info.boost_rate,
        }
    }
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

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PoolStatus {
    Inactive,
    Activing,
    // Suspended: Due to special circumstances, new staking are suspended, and old staking continue to be maintained
    Suspended,
    // Terminated: This staking pool is terminated due to special reasons,
    // and the BTC that has been staking will be refunded by the original route
    Terminated,
    Completed,
}

/// Parse string to PoolStatus, if the string is unknown, it's `inactive`
impl From<String> for PoolStatus {
    fn from(value: String) -> Self {
        match value.trim().to_lowercase().as_str() {
            "inactive" => PoolStatus::Inactive,
            "activing" => PoolStatus::Activing,
            "suspended" => PoolStatus::Suspended,
            "terminated" => PoolStatus::Terminated,
            "completed" => PoolStatus::Completed,
            _ => PoolStatus::Inactive,
        }
    }
}

impl Display for PoolStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = format!("{:?}", self);
        write!(f, "{}", v.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn staking_status_should_works() {
        let pending = StakingStatus::Pending;
        let confirmed = StakingStatus::Confirmed;
        let redeeming = StakingStatus::Redeeming;
        let redeemed = StakingStatus::Redeemed;

        assert!(!pending.after(&pending));
        assert!(confirmed.after(&pending));
        assert!(redeeming.after(&pending));
        assert!(redeemed.after(&pending));

        assert!(!confirmed.after(&confirmed));
        assert!(!pending.after(&confirmed));
        assert!(redeeming.after(&confirmed));
        assert!(redeemed.after(&confirmed));

        assert!(redeemed.after(&redeeming));
    }
}
