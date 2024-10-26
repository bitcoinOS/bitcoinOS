use crate::repositories;
use wallet::domain::staking::StakingRecord;

pub(super) fn serve(wallet: String) -> Vec<StakingRecord> {
    repositories::staking_record::list_staking_records_by_wallet(wallet)
}
