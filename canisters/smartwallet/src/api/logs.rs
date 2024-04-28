use crate::{context::STATE, domain::TransactionLog};

/// Returns all addresses of this canister
pub(super) async fn serve() -> Vec<TransactionLog> {
    STATE.with(|s| s.borrow().logs.iter().collect())
}
