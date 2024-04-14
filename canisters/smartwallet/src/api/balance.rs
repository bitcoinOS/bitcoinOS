use candid::Principal;
use ic_cdk::api::management_canister::bitcoin::Satoshi;

use crate::context::STATE;
use crate::error::WalletError;

/// Returns the balance of the given bitcoin address
pub(super) async fn serve(address: String, caller: Principal) -> Result<Satoshi, WalletError> {
    let metadata = STATE.with(|s| s.borrow().metadata.get().clone());

    if caller != metadata.owner {
        return Err(WalletError::UnAuthorized(caller.to_string()));
    }

    let network = metadata.network;
    base::utils::balance(address, network)
        .await
        .map_err(|e| e.into())
}
