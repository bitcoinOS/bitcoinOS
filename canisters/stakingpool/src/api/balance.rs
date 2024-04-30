use ic_cdk::api::management_canister::bitcoin::Satoshi;

use crate::{domain::Metadata, error::WalletError};

/// Returns the balance of the given bitcoin address
pub(super) async fn serve(address: String, metadata: Metadata) -> Result<Satoshi, WalletError> {
    wallet::bitcoins::balance(address, metadata.network)
        .await
        .map_err(|e| e.into())
}
