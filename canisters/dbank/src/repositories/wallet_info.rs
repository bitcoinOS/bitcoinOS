use crate::{
    context::STATE,
    domain::{DBankWalletInfo, SelfCustodyKey},
    // error::DBankError,
};

pub(crate) fn count() -> u64 {
    STATE.with(|s| s.borrow().wallet_infos.len())
}

pub(crate) fn list_wallet() -> Vec<DBankWalletInfo> {
    let mut wallets: Vec<_> =
        STATE.with_borrow(|s| s.wallet_infos.iter().map(|(_, w)| w).collect());
    wallets.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    wallets
}

/// Find the wallet info list by owner
pub(crate) fn get_info(key: &SelfCustodyKey) -> Option<DBankWalletInfo> {
    STATE.with_borrow(|s| s.wallet_infos.get(key))
}
