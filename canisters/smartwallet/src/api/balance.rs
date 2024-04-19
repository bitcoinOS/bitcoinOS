use candid::Principal;
use ic_cdk::api::management_canister::bitcoin::Satoshi;

use crate::error::WalletError;

use super::validate_controller;

/// Returns the balance of the given bitcoin address
pub(super) async fn serve(address: String, caller: Principal) -> Result<Satoshi, WalletError> {
    let network = validate_controller(caller).map(|m| m.network)?;

    base::bitcoins::balance(address, network)
        .await
        .map_err(|e| e.into())
}
