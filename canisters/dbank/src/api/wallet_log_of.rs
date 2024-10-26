use crate::{domain::WalletOperationEvent, repositories};

/// Returns a wallet log of given index
pub(super) fn serve(index: u64) -> Option<WalletOperationEvent> {
    repositories::wallet_logs::get(index)
}
