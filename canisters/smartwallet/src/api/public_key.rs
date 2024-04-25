use base::utils::{hex, principal_to_derivation_path};
use candid::Principal;

use crate::{
    domain::{response::PublicKeyResponse, Metadata},
    error::WalletError,
};

pub async fn serve(owner: Principal, metadata: Metadata) -> Result<PublicKeyResponse, WalletError> {
    base::ecdsa::public_key(
        principal_to_derivation_path(owner),
        metadata.ecdsa_key_id,
        None,
    )
    .await
    .map_err(|e| e.into())
    .map(|res| PublicKeyResponse {
        public_key_hex: hex(res),
    })
}
