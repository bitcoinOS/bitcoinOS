use ic_cdk::api::management_canister::{bitcoin::Satoshi, main::CanisterId};

use crate::{
    context::STATE,
    domain::{StakingRecord, StakingStatus, TxID},
    error::StakingError,
};

/// Save staking record if it doesn't exist, otherwise return StakingRecordAlreadyExists
pub fn save(record: &StakingRecord) -> Result<(), StakingError> {
    STATE.with(|s| {
        let records = &mut s.borrow_mut().staking_records;
        let key = record.txid.clone();
        if records.contains_key(&key) {
            Err(StakingError::StakingRecordAlreadyExists(key))
        } else {
            records.insert(key, record.to_owned());
            Ok(())
        }
    })
}

fn update_status(
    txid: TxID,
    status: StakingStatus,
    updated_time: u64,
    received_amount: Option<Satoshi>,
) -> Result<(), StakingError> {
    STATE.with(|s| {
        let records = &mut s.borrow_mut().staking_records;

        match records.get(&txid) {
            Some(record) => {
                let new_record = StakingRecord {
                    updated_time,
                    status,
                    received_amount: received_amount.unwrap_or(record.received_amount),
                    ..record
                };

                records.insert(txid, new_record);
                Ok(())
            }
            _ => Err(StakingError::StakingRecordNotFound(txid)),
        }
    })
}

pub fn confirmed_record(
    txid: TxID,
    received_amount: Satoshi,
    updated_time: u64,
) -> Result<(), StakingError> {
    update_status(
        txid,
        StakingStatus::Confirmed,
        updated_time,
        Some(received_amount),
    )
}

pub fn redeeming_record(txid: TxID, updated_time: u64) -> Result<(), StakingError> {
    update_status(txid, StakingStatus::Redeeming, updated_time, None)
}

pub fn redeemed_record(txid: TxID, updated_time: u64) -> Result<(), StakingError> {
    update_status(txid, StakingStatus::Redeemed, updated_time, None)
}

/// Validate the sender is the staker and the amount is valid
pub fn validate_staker_amount(
    staker: CanisterId,
    txid: &TxID,
    redeem_time: u64,
) -> Result<u64, StakingError> {
    STATE.with(|s| {
        let records = &s.borrow().staking_records;
        match records.get(txid) {
            Some(record) => {
                if record.sender == staker
                    && record.sent_time + record.duration_in_millisecond < redeem_time
                    && record.status == StakingStatus::Confirmed
                {
                    let amount = record.received_amount + calculate_interest();
                    Ok(amount)
                } else {
                    Err(StakingError::RedemptionNotAllowed)
                }
            }
            _ => Err(StakingError::StakingRecordNotFound(txid.to_owned())),
        }
    })
}

/// TODO: Calculate interest with duration and interest rate
fn calculate_interest() -> Satoshi {
    0
}
