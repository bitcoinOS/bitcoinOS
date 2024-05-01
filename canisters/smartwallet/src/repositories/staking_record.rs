use crate::{context::STATE, domain::StakingRecord, error::WalletError};

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
