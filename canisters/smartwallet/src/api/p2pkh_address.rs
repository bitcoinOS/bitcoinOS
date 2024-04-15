use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, ecdsa::EcdsaKeyId};

use crate::error::WalletError;

pub(super) async fn serve(
    network: BitcoinNetwork,
    derivation_path: Vec<Vec<u8>>,
    key_id: EcdsaKeyId,
) -> Result<String, WalletError> {
    base::bitcoins::get_p2pkh_address(network, derivation_path, key_id, None)
        .await
        .map_err(|e| e.into())
}
