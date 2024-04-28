use crate::context::RAW_WALLET;

/// Returns all addresses of this canister
pub(super) async fn serve() -> Vec<String> {
    RAW_WALLET.with(|w| w.borrow().iter().map(|(_, ra)| ra.address).collect())
}
