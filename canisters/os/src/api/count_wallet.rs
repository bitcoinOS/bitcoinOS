use crate::{services, WALLET_OWNER};

pub fn serve() -> u64 {
    WALLET_OWNER.with(|w| services::count_wallet::execute(&w.into()))
}
