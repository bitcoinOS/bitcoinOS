use wallet::domain::{staking::StakingRecord, TxId};

use crate::repositories;

pub(super) fn serve(txid: &TxId) -> Option<StakingRecord> {
    repositories::staking_record::get_staking(txid)
}
