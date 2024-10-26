use ic_cdk::api::management_canister::main::CanisterId;
use wallet::domain::{staking::StakingRecord, TxId};

use crate::{context::STATE, error::Error};

pub(crate) fn save(record: StakingRecord) -> Result<(), Error> {
    STATE.with_borrow_mut(|s| {
        let records = &mut s.staking_records;
        let key = record.txid.clone();
        if records.contains_key(&key) {
            Err(Error::StakingRecordAlreadyExists(key))
        } else {
            records.insert(key, record);
            Ok(())
        }
    })
}

pub(crate) fn total_staking() -> u64 {
    STATE.with_borrow(|s| {
        s.staking_records
            .iter()
            .filter(|(_, r)| r.is_tvl())
            .map(|(_, r)| r.sent_amount)
            .sum()
    })
}

pub(crate) fn total_staking_by_pool(staking_canister: CanisterId) -> u64 {
    STATE.with_borrow(|s| {
        s.staking_records
            .iter()
            .filter(|(_, r)| r.staking_canister == staking_canister && r.is_tvl())
            .map(|(_, r)| r.sent_amount)
            .sum()
    })
}

/// Get staking record by txid
pub(crate) fn get(txid: &TxId) -> Option<StakingRecord> {
    STATE.with_borrow(|s| s.staking_records.get(txid))
}

pub(crate) fn list() -> Vec<StakingRecord> {
    STATE.with_borrow(|s| {
        s.staking_records
            .iter()
            .map(|(_, r)| r.to_owned())
            .collect()
    })
}
