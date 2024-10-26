use ic_cdk::api::management_canister::main::CanisterId;
use wallet::utils::ic_time;

use crate::repositories;
use wallet::domain::request::RegisterStakingRecordRequest;
use wallet::domain::staking::{StakingRecord, StakingStatus};
use wallet::error::StakingError;

pub(crate) fn serve(
    sender_canister: CanisterId,
    updated_time: u64,
    req: RegisterStakingRecordRequest,
    interest_rate: u16,
    duration_in_ms: u64,
    staking_canister: CanisterId,
    staking_address: String,
) -> Result<StakingRecord, StakingError> {
    let txid = req.txid;

    let record = StakingRecord {
        txid,
        sender: req.sender,
        sender_canister,
        sender_address: req.sender_address,
        sent_amount: req.sent_amount,
        sent_time: ic_time(),
        annual_interest_rate: interest_rate,
        duration_in_day: duration_in_ms,
        network: req.network,
        staking_canister,
        staking_address,
        actual_amount: 0,
        status: StakingStatus::Pending,
        redeemed_txid: None,
        updated_time,
        memo: req.memo,
        stake_type: req.stake_type,
        fund_management: req.fund_management.into(),
    };

    repositories::staking_record::save(&record)?;

    Ok(record)
}
