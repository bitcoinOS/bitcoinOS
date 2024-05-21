use ic_cdk::api::management_canister::main::CanisterId;
use wallet::domain::staking::{StakingRecord, TxId};

use crate::{context::STATE, error::WalletError};

pub(crate) fn save(record: StakingRecord) -> Result<(), WalletError> {
    STATE.with(|s| {
        let records = &mut s.borrow_mut().staking_records;
        let key = record.txid.clone();
        if records.contains_key(&key) {
            Err(WalletError::StakingRecordAlreadyExists(key))
        } else {
            records.insert(key, record);
            Ok(())
        }
    })
}

pub(crate) fn total_staking(address: String, staking_canister: CanisterId) -> u64 {
    STATE.with_borrow(|s| {
        s.staking_records
            .iter()
            .filter(|(_, r)| r.sender_address == address && r.staking_canister == staking_canister)
            .map(|(_, r)| r.sent_amount)
            .sum()
    })
}

/// Get staking record by txid
pub(crate) fn get_staking(txid: &TxId) -> Option<StakingRecord> {
    STATE.with_borrow(|s| s.staking_records.get(txid))
}

pub(crate) fn list_staking() -> Vec<StakingRecord> {
    STATE.with_borrow(|s| {
        s.staking_records
            .iter()
            .map(|(_, r)| r.to_owned())
            .collect()
    })
}

pub(crate) fn update(pool_record: StakingRecord) -> Result<(), WalletError> {
    STATE.with_borrow_mut(|s| match s.staking_records.get(&pool_record.txid) {
        Some(local_record) => {
            if local_record.can_update(&pool_record) {
                s.staking_records
                    .insert(pool_record.txid.clone(), pool_record);
                Ok(())
            } else {
                Err(WalletError::StakingRecordCantUpdate(
                    pool_record.txid.clone(),
                ))
            }
        }
        None => {
            s.staking_records
                .insert(pool_record.txid.clone(), pool_record);
            Ok(())
        }
    })
}
