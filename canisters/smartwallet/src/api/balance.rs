use ic_cdk::api::management_canister::bitcoin::Satoshi;

use crate::context::STATE;
use crate::error::WalletError;

/// Returns the balance of the given bitcoin address
pub(super) async fn serve(address: String) -> Result<Satoshi, WalletError> {
    let network = STATE.with(|s| s.borrow().metadata.get().network);
    base::utils::balance(address, network)
        .await
        .map_err(|e| e.into())
}
