use crate::repositories;
use wallet::domain::staking::StakingRecord;

pub(super) fn serve() -> Vec<StakingRecord> {
    repositories::staking_record::list_staking_records()
}
