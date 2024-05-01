use crate::{error::Error, repositories};

pub(crate) fn serve() -> Result<u128, Error> {
    repositories::wallet_counter::increment_one()
}
