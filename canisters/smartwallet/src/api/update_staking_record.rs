use crate::{domain::StakingRecord, error::WalletError, repositories};

pub(crate) fn serve(info: StakingRecord) -> Result<(), WalletError> {
    repositories::staking_record::update(info)
}
