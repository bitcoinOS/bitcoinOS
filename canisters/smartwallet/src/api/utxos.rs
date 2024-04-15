use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, GetUtxosResponse};

use crate::error::WalletError;

pub async fn serve(
    address: String,
    network: BitcoinNetwork,
) -> Result<GetUtxosResponse, WalletError> {
    base::bitcoins::wallet::get_utxos(address, network, None)
        .await
        .map_err(|e| e.into())
}
