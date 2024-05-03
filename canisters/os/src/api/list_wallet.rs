use crate::{domain::WalletInfo, repositories};

pub fn serve() -> Vec<WalletInfo> {
    repositories::wallet_info::list_wallet()
}
