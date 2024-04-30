use ic_cdk::api::management_canister::main::CanisterId;

use crate::{
    domain::{request::RegisterStakingRequest, StakingRecord, StakingStatus},
    error::StakingError,
    repositories,
};

pub(crate) async fn serve(
    sender: CanisterId,
    updated_time: u64,
    req: RegisterStakingRequest,
    duration_in_ms: u64,
) -> Result<StakingRecord, StakingError> {
    let txid = req.txid;

    let record = StakingRecord {
        txid,
        sender,
        sender_address: req.sender_address,
        sent_amount: req.sent_amount,
        sent_time: req.sent_time,
        duration_in_millisecond: duration_in_ms,
        network: req.network,
        received_amount: 0,
        status: StakingStatus::Pending,
        updated_time,
    };

    repositories::staking_record::save(&record)?;

    Ok(record)
}
