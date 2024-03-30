mod get_ecdsa_key;
mod register_ecdsa_key;
mod update_ecdsa_key;
mod update_public_key;

use base::utils::validate_network;
use candid::Principal;
use ic_cdk::{export_candid, init, query, update};

use crate::context::METADATA;
use crate::{
    domain::{Metadata, UpdateKeyRequest},
    error::StewardError,
};

#[query]
pub fn get_ecdsa_key() -> Result<String, StewardError> {
    let caller = ic_caller();
    get_ecdsa_key::serve(&caller)
}

#[update]
pub fn register_ecdsa_key(key: String) -> Result<bool, StewardError> {
    let caller = ic_caller();
    let updated_time = ic_time();

    register_ecdsa_key::serve(caller, key, updated_time)
}

#[update]
pub fn update_ecdsa_key(req: UpdateKeyRequest) -> Result<bool, StewardError> {
    let caller = ic_caller();
    let updated_time = ic_time();

    update_ecdsa_key::serve(caller, req.new_key, req.old_key, updated_time)
}

#[query]
fn metadata() -> Metadata {
    METADATA.with(|m| m.borrow().get().clone())
}

#[init]
fn init(network: String) {
    METADATA.with(|m| {
        let mut metadata = m.borrow_mut();
        metadata
            .set(Metadata {
                network: validate_network(&network),
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
