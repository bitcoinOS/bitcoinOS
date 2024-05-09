use crate::{error::Error, repositories};

pub(super) fn serve(cycle_value: u64) -> Result<u64, Error> {
    repositories::metadata::set_wallet_cycles(cycle_value)
}
