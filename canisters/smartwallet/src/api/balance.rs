use candid::Principal;
use ic_cdk::api::management_canister::bitcoin::Satoshi;

use crate::error::WalletError;

use super::get_metadata;

/// Returns the balance of the given bitcoin address
pub(super) async fn serve(address: String, caller: Principal) -> Result<Satoshi, WalletError> {
    let metadata = get_metadata();

    if caller != metadata.owner {
        return Err(WalletError::UnAuthorized(caller.to_string()));
    }

    let network = metadata.network;
    base::bitcoins::balance(address, network)
        .await
        .map_err(|e| e.into())
}
