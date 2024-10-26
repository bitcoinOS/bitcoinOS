use crate::{domain::TransactionLog, repositories};

/// Returns a specific transaction log for a given index
pub(super) fn serve(index: u64) -> Option<TransactionLog> {
    repositories::tx_logs::get(index)
}
