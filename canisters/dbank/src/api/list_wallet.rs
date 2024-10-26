use crate::{domain::DBankWalletInfo, repositories};

pub(super) fn serve() -> Vec<DBankWalletInfo> {
    repositories::wallet_info::list_wallet()
}
