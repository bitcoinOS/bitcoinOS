use candid::Principal;
use ic_cdk::api::management_canister::ecdsa::{
    EcdsaKeyId, EcdsaPublicKeyArgument, SignWithEcdsaArgument,
};

use crate::error::Error;

/// Returns the ECDSA public key of this canister at the given derivation path.
pub async fn public_key(
    derivation_path: Vec<Vec<u8>>,
    key_id: EcdsaKeyId,
    canister_id: Option<Principal>,
) -> Result<Vec<u8>, Error> {
    let arg = EcdsaPublicKeyArgument {
        derivation_path,
        key_id,
        canister_id,
    };

    // Retrieve public key of this canister with the given derivation path from ic management canister
    let resp = ic_cdk::api::management_canister::ecdsa::ecdsa_public_key(arg).await;

    resp.map(|r| r.0.public_key).map_err(|e| e.into())
}

/// Signs a message with IC ECDSA interfaces
pub async fn sign_with_ecdsa(
    derivation_path: Vec<Vec<u8>>,
    key_id: EcdsaKeyId,
    message_hash: Vec<u8>,
) -> Result<Vec<u8>, Error> {
    let arg = SignWithEcdsaArgument {
        derivation_path,
        key_id,
        message_hash,
    };

    let resp = ic_cdk::api::management_canister::ecdsa::sign_with_ecdsa(arg).await;

    resp.map(|r| r.0.signature).map_err(|e| e.into())
}
