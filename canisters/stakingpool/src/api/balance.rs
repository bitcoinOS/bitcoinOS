use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, Satoshi};

use crate::error::StakingError;

/// Returns the balance of the given bitcoin address
pub(super) async fn serve(
    address: String,
    network: BitcoinNetwork,
) -> Result<Satoshi, StakingError> {
    wallet::bitcoins::balance(address, network)
        .await
        .map_err(|e| e.into())
}
