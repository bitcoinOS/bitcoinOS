use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, UtxoFilter};
use wallet::domain::response::UtxosResponse;

use wallet::error::StakingError;

pub async fn serve(
    address: String,
    network: BitcoinNetwork,
    filter: Option<UtxoFilter>,
) -> Result<UtxosResponse, StakingError> {
    wallet::bitcoins::get_utxos(address, network, filter)
        .await
        .map_err(|e| e.into())
}
