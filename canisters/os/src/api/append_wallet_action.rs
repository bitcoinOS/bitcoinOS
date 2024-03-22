use candid::Principal;

use crate::{domain::Action, error::Error, services, WALLET_ACTION};

pub fn serve(operator: Principal, action: Action, op_time: u64) -> Result<u64, Error> {
    WALLET_ACTION
        .with(|w| services::append_wallet_action::execute(w.into(), operator, action, op_time))
}
