use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, GetUtxosResponse, UtxoFilter};

use crate::error::WalletError;

pub async fn serve(
    address: String,
    network: BitcoinNetwork,
    filter: Option<UtxoFilter>,
) -> Result<GetUtxosResponse, WalletError> {
    wallet::bitcoins::get_utxos(address, network, filter)
        .await
        .map_err(|e| e.into())
}
