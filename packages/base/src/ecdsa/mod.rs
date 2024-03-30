use candid::Principal;

use crate::{
    domain::{ECDSAPublicKey, ECDSAPublicKeyReply, EcdsaCurve, EcdsaKeyId},
    error::Error,
};

/// Returns the ECDSA public key of this canister at the given derivation path.
pub async fn public_key(
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    canister_id: Option<Principal>,
) -> Result<Vec<u8>, Error> {
    // Retrieve public key of this canister with the given derivation path from ic management canister
    let resp: Result<(ECDSAPublicKeyReply,), _> = ic_cdk::call(
        Principal::management_canister(),
        "ecdsa_public_key",
        (ECDSAPublicKey {
            canister_id,
            derivation_path,
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: key_name,
            },
        },),
    )
    .await;

    resp.map(|r| r.0.public_key).map_err(|e| e.into())
}
