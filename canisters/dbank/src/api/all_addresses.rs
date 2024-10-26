use crate::context::STATE;

/// Returns all addresses of this canister
pub(super) async fn serve() -> Vec<String> {
    STATE.with(|s| {
        s.borrow()
            .wallets
            .iter()
            .map(|(_, ra)| ra.address)
            .collect()
    })
}
