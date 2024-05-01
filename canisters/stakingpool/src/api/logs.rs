use crate::{context::STATE, domain::RedeemLog};

/// Returns all addresses of this canister
pub(super) async fn serve() -> Vec<RedeemLog> {
    STATE.with(|s| s.borrow().redeem_logs.iter().collect())
}
