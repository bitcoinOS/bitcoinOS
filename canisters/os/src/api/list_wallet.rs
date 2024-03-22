use crate::{domain::WalletOwner, services, WALLET_OWNER};

pub fn serve() -> Vec<WalletOwner> {
    WALLET_OWNER.with(|w| services::list_wallet::execute(&w.into()))
}
