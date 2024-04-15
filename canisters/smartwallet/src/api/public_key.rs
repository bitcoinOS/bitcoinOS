use base::utils::hex;
use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;

use crate::{domain::response::PublicKeyResponse, error::WalletError};

pub async fn serve(
    derivation_path: Vec<Vec<u8>>,
    key_id: EcdsaKeyId,
) -> Result<PublicKeyResponse, WalletError> {
    base::ecdsa::public_key(derivation_path, key_id, None)
        .await
        .map_err(|e| e.into())
        .map(|res| PublicKeyResponse {
            public_key_hex: hex(res),
        })
}
