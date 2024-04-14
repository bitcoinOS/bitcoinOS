use candid::Principal;
use ic_cdk::api::management_canister::ecdsa::{
    EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument, EcdsaPublicKeyResponse,
};

use crate::{
    constants::SIGN_WITH_ECDSA_COST_CYCLES,
    domain::{SignWithEcdsa, SignWithEcdsaReply},
    error::Error,
    utils::{call_management_with_payment, mgmt_canister_id},
};

/// Returns the ECDSA public key of this canister at the given derivation path.
pub async fn public_key(
    key_name: &str,
    derivation_path: Vec<Vec<u8>>,
    canister_id: Option<Principal>,
) -> Result<Vec<u8>, Error> {
    // Retrieve public key of this canister with the given derivation path from ic management canister
    let resp: Result<(EcdsaPublicKeyResponse,), _> = ic_cdk::call(
        mgmt_canister_id(),
        "ecdsa_public_key",
        (EcdsaPublicKeyArgument {
            canister_id,
            derivation_path,
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: key_name.to_string(),
            },
        },),
    )
    .await;

    resp.map(|r| r.0.public_key).map_err(|e| e.into())
}

/// Signs a message with IC ECDSA interfaces
pub async fn sign_with_ecdsa(
    key_name: &str,
    derivation_path: Vec<Vec<u8>>,
    message_hash: Vec<u8>,
) -> Result<Vec<u8>, Error> {
    let resp: Result<(SignWithEcdsaReply,), _> = call_management_with_payment(
        "sign_with_ecdsa",
        (SignWithEcdsa {
            message_hash,
            derivation_path,
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: key_name.to_string(),
            },
        },),
        SIGN_WITH_ECDSA_COST_CYCLES,
    )
    .await;

    resp.map(|r| r.0.signature).map_err(|e| e.into())
}
