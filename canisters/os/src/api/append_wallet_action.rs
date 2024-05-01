use candid::Principal;

use crate::{domain::Action, error::Error, repositories};

pub fn serve(operator: Principal, action: Action, op_time: u64) -> Result<u64, Error> {
    repositories::wallet_log::append_wallet_action(operator, action, op_time)
}
