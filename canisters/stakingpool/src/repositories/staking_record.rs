use ic_cdk::api::management_canister::{bitcoin::Satoshi, main::CanisterId};

use crate::{constants::DAY_IN_NANOSECOND, context::STATE};
use wallet::domain::{
    staking::{StakingRecord, StakingStatus},
    TxId,
};
use wallet::error::StakingError;
/// Get staking record by txid
pub(crate) fn get_staking(txid: TxId) -> Option<StakingRecord> {
    STATE.with_borrow(|s| s.staking_records.get(&txid))
}

/// List all staking records
pub(crate) fn list_staking_records() -> Vec<StakingRecord> {
    STATE.with_borrow(|s| s.staking_records.iter().map(|(_, r)| r).collect())
}

/// List all staking records by wallet address
pub(crate) fn list_staking_records_by_wallet(wallet: String) -> Vec<StakingRecord> {
    STATE.with_borrow(|s| {
        s.staking_records
            .iter()
            .map(|(_, r)| r)
            .filter(|r| r.sender_address == wallet)
            .collect()
    })
}

/// Calculate the total amount staked
/// TODO: FIX using `actual_amount`
pub(crate) fn sum_staking_amount() -> Satoshi {
    STATE.with_borrow(|s| {
        s.staking_records
            .iter()
            .filter(|(_, r)| {
                r.status == StakingStatus::Confirmed || r.status == StakingStatus::Pending
            })
            .map(|(_, r)| r.sent_amount)
            .sum()
    })
}

/// Save staking record if it doesn't exist, otherwise return StakingRecordAlreadyExists
pub(crate) fn save(record: &StakingRecord) -> Result<(), StakingError> {
    STATE.with_borrow_mut(|s| {
        let records = &mut s.staking_records;
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
    txid: TxId,
    status: StakingStatus,
    updated_time: u64,
    received_amount: Option<Satoshi>,
    redeemed_txid: Option<TxId>,
) -> Result<(), StakingError> {
    STATE.with_borrow_mut(|s| {
        let records = &mut s.staking_records;

        match records.get(&txid) {
            Some(record) => {
                let new_record = StakingRecord {
                    updated_time,
                    status,
                    actual_amount: received_amount.unwrap_or(record.actual_amount),
                    redeemed_txid,
                    ..record
                };

                records.insert(txid, new_record);
                Ok(())
            }
            _ => Err(StakingError::StakingRecordNotFound(txid)),
        }
    })
}

// pub(crate) fn confirmed_record(
//     txid: TxId,
//     received_amount: Satoshi,
//     updated_time: u64,
// ) -> Result<(), StakingError> {
//     update_status(
//         txid,
//         StakingStatus::Confirmed,
//         updated_time,
//         Some(received_amount),
//         None,
//     )
// }

pub(crate) fn redeeming_record(txid: TxId, updated_time: u64) -> Result<(), StakingError> {
    update_status(txid, StakingStatus::Redeeming, updated_time, None, None)
}

pub(crate) fn redeemed_record(
    txid: TxId,
    updated_time: u64,
    redeem_txid: TxId,
) -> Result<(), StakingError> {
    update_status(
        txid,
        StakingStatus::Redeemed,
        updated_time,
        None,
        Some(redeem_txid),
    )
}

/// Validate the sender is the staker and the amount is valid
pub(crate) fn validate_staker_amount(
    staker: CanisterId,
    txid: &TxId,
    redeem_time: u64,
) -> Result<u64, StakingError> {
    STATE.with_borrow(|s| {
        let records = &s.staking_records;
        match records.get(txid) {
            Some(record) => {
                if record.sender == staker
                    && record.sent_time + (record.duration_in_day * DAY_IN_NANOSECOND) < redeem_time
                    && record.status == StakingStatus::Confirmed
                {
                    let amount = record.actual_amount
                        + calculate_interest(
                            record.actual_amount,
                            record.annual_interest_rate,
                            record.duration_in_day,
                        );
                    Ok(amount)
                } else {
                    Err(StakingError::RedemptionNotAllowed)
                }
            }
            _ => Err(StakingError::StakingRecordNotFound(txid.to_owned())),
        }
    })
}

/// Calculate interest with duration and interest rate
fn calculate_interest(amount: Satoshi, interest_rate: u16, duration_in_day: u64) -> Satoshi {
    amount * interest_rate as u64 * duration_in_day / (10000 * 365)
}
