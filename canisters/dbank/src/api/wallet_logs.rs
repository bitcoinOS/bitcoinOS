use crate::{domain::WalletOperationEvent, repositories};

/// Returns all wallet operation logs
pub(super) async fn serve() -> Vec<WalletOperationEvent> {
    repositories::wallet_logs::list()
}
