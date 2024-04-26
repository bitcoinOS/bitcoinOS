use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;

use crate::error::StewardError;

pub async fn serve(
    derivation_path: Vec<Vec<u8>>,
    key_id: EcdsaKeyId,
) -> Result<Vec<u8>, StewardError> {
    base::ecdsa::public_key(derivation_path, key_id, None)
        .await
        .map_err(|e| e.into())
}
