use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::{
    bitcoin::{BitcoinNetwork, Satoshi},
    main::CanisterId,
};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

use super::TxId;

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
