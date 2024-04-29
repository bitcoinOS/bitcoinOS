use crate::{error::Error, repositories};

pub(crate) fn serve() -> Result<u128, Error> {
    repositories::staking_pool_counter::counter_increment_one()
}
