use crate::{domain::WalletOwner, repositories};

pub fn serve() -> Vec<WalletOwner> {
    repositories::wallet_owner::list_wallet()
}
