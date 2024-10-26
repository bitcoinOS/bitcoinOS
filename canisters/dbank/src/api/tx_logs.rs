use crate::{domain::TransactionLog, repositories};

/// Returns all transaction logs
pub(super) async fn serve() -> Vec<TransactionLog> {
    repositories::tx_logs::list()
}
