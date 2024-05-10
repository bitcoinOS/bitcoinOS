use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use wallet::domain::response::UtxosResponse;

use crate::error::WalletError;

pub async fn serve(address: String, network: BitcoinNetwork) -> Result<UtxosResponse, WalletError> {
    wallet::bitcoins::get_utxos(address, network, None)
        .await
        .map_err(|e| e.into())
}
