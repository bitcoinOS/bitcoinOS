use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;

use crate::error::WalletError;

pub(super) async fn serve(
    network: BitcoinNetwork,
    key_name: &str,
    derivation_path: Vec<Vec<u8>>,
) -> Result<String, WalletError> {
    base::bitcoins::wallet::get_p2pkh_address(network, key_name, derivation_path)
        .await
        .map_err(|e| e.into())
}
