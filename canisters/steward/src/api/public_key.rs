use base::utils::hex;

use crate::{domain::response::PublicKeyResponse, error::StewardError};

pub async fn serve(
    key_name: &str,
    derivation_path: Vec<Vec<u8>>,
) -> Result<PublicKeyResponse, StewardError> {
    base::ecdsa::public_key(key_name, derivation_path, None)
        .await
        .map_err(|e| e.into())
        .map(|res| PublicKeyResponse {
            public_key_hex: hex(res),
        })
}
