use crate::{error::Error, repositories};

use wallet::domain::staking::StakingRecord;

pub(crate) fn serve(record: StakingRecord) -> Result<StakingRecord, Error> {
    repositories::staking_record::save(record.clone())?;

    Ok(record)
}
