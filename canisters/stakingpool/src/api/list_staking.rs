use crate::{domain::StakingRecord, repositories};

pub(super) fn serve() -> Vec<StakingRecord> {
    repositories::staking_record::list_staking_records()
}
