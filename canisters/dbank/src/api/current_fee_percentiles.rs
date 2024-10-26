use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, MillisatoshiPerByte};

use crate::error::DBankError;

pub(super) async fn serve(network: BitcoinNetwork) -> Result<Vec<MillisatoshiPerByte>, DBankError> {
    wallet::bitcoins::get_current_fee_percentiles(network)
        .await
        .map_err(|e| e.into())
}
