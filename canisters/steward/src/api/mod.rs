mod get_ecdsa_key;
mod register_ecdsa_key;
mod update_ecdsa_key;
mod update_public_key;

use base::utils::to_ic_bitcoin_network;
use candid::Principal;
use ic_cdk::{export_candid, init, query, update};

use crate::context::METADATA;
use crate::{
    domain::{Metadata, UpdateKeyRequest},
    error::StewardError,
};

/// --------------------- Update interface of this Canister ----------------------
///
/// Register ecdsa key of caller(wallet canister)
/// Returns Ok(true) if success, otherwise returns ECDSAKeyAlreadyExists error
#[update]
pub async fn register_ecdsa_key(key: String) -> Result<bool, StewardError> {
    let caller = ic_caller();
    let updated_time = ic_time();

    register_ecdsa_key::serve(caller, key, updated_time)
}

/// Update ecdsa key of this canister if old key is match and caller is match wallet canister
/// Returns Ok(true) if success, otherwise returns ECDSAKeyNotFound or ECDSAKeyUpdateError
#[update]
pub async fn update_ecdsa_key(req: UpdateKeyRequest) -> Result<bool, StewardError> {
    let caller = ic_caller();
    let updated_time = ic_time();

    update_ecdsa_key::serve(caller, req.new_key, req.old_key, updated_time)
}

#[update]
pub async fn public_key(
    derivation_path: Vec<Vec<u8>>,
) -> Result<Vec<u8>, StewardError> {
    let caller = ic_caller();
    let key_name = get_ecdsa_key::serve(&caller)?;
    base::ecdsa::public_key(key_name, derivation_path, Some(caller))
        .await
        .map_err(|e| e.into())
}

/// --------------------- Queries interface of this canister -------------------
///
/// Returns ecdsa key if caller already registered
/// otherwise retunrs ECDSAKeyNotFound
#[query]
pub fn get_ecdsa_key() -> Result<String, StewardError> {
    let caller = ic_caller();
    get_ecdsa_key::serve(&caller)
}

/// Returns this canister's metadata
#[query]
fn metadata() -> Metadata {
    METADATA.with(|m| m.borrow().get().clone())
}

/// Init canister with `network` argument
#[init]
fn init(network: String) {
    METADATA.with(|m| {
        let mut metadata = m.borrow_mut();
        metadata
            .set(Metadata {
                network: to_ic_bitcoin_network(&network),
            })
            .expect("Failed to init network")
    });
}

export_candid!();

fn ic_caller() -> Principal {
    ic_cdk::caller()
}

fn ic_time() -> u64 {
    ic_cdk::api::time()
}
