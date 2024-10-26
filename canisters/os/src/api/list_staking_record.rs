use wallet::domain::staking::StakingRecord;

use crate::repositories;

pub(super) fn serve() -> Vec<StakingRecord> {
    repositories::staking_record::list()
}
