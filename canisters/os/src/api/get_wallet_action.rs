use crate::{domain::WalletAction, repositories};

pub fn serve(idx: u64) -> Option<WalletAction> {
    repositories::wallet_log::get_wallet_action(idx)
}
