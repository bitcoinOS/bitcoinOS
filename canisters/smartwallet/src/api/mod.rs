mod get_ecdsa_key;
mod register_ecdsa_key;
mod update_ecdsa_key;

use candid::Principal;
use ic_cdk::{query, update};

use crate::context::{State, STATE};
use crate::domain::response::NetworkResponse;
use crate::domain::{request::UpdateKeyRequest, Metadata};
use crate::error::WalletError;

/// ---------------- Update interface of this canister ------------------
///
/// Get an exists address for the caller,
/// or create a new one if it doesn't exist
#[update]
pub async fn get_or_create_wallet_address() -> Result<String, WalletError> {
    let caller = ic_caller();

    crate::bitcoin::get_or_create_wallet_address(caller).await
}

/// Register ecdsa key of this canister if the caller is controller and the key donesn't exists
/// Returns true if success, or returns `RegisterECDSAKeyError`
/// otherwise return `EcdsaKeyAlreadyExists` or `UnAuthorized`
#[update]
pub fn register_ecdsa_key(key: String) -> Result<bool, WalletError> {
    let caller = ic_caller();
    let updated_time = ic_time();

    register_ecdsa_key::serve(&caller, key, updated_time)
}

/// Update ecdsa key of this canister if the caller is controller and the key exists
/// Returns true if success, or returns `EcdsaKeyUpdateError`
/// otherwise return `UnAuthorized`
#[update]
pub fn update_ecdsa_key(req: UpdateKeyRequest) -> Result<bool, WalletError> {
    let caller = ic_caller();
    let updated_time = ic_time();

    update_ecdsa_key::serve(&caller, req.new_key, req.old_key, updated_time)
}

/// --------------------- Queries interface of this canister -------------------
/// Returns ecdsa key of this canister if the caller is controller and the key exists
/// otherwise return `EcdsaKeyNotFound` or `UnAuthorized`
#[query]
pub fn get_ecdsa_key() -> Result<String, WalletError> {
    let caller = ic_caller();
    get_ecdsa_key::serve(&caller)
}

/// Returns the network of this canister
#[query]
fn network() -> NetworkResponse {
    STATE.with(|s| s.borrow().metadata.get().network.into())
}

/// Returns the metadata of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
fn metadata() -> Result<Metadata, WalletError> {
    let caller = ic_caller();
    STATE.with(|s| {
        let state = s.borrow();
        validate_controller(&state, &caller, |s| Ok(s.metadata.get().clone()))
    })
}

/// Returns the controllers of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
fn controller() -> Result<Vec<Principal>, WalletError> {
    let caller = ic_caller();

    STATE.with(|s| {
        let state = s.borrow();

        validate_controller(&state, &caller, |s| match s.controllers.get(&caller) {
            Some(_) => Ok(state.controllers.iter().map(|(k, _)| k).collect()),
            None => Err(WalletError::UnAuthorized(caller.to_string())),
        })
    })
}

fn ic_caller() -> Principal {
    ic_cdk::caller()
}

fn ic_time() -> u64 {
    ic_cdk::api::time()
}

fn validate_controller<F, T>(state: &State, caller: &Principal, f: F) -> Result<T, WalletError>
where
    F: FnOnce(&State) -> Result<T, WalletError>,
{
    match state.controllers.get(caller) {
        Some(_) => f(state),
        None => Err(WalletError::UnAuthorized(caller.to_string())),
    }
}

fn validate_controller_mut<F, T>(
    state: &mut State,
    caller: &Principal,
    mut f: F,
) -> Result<T, WalletError>
where
    F: FnMut(&mut State) -> Result<T, WalletError>,
{
    match state.controllers.get(caller) {
        Some(_) => f(state),
        None => Err(WalletError::UnAuthorized(caller.to_string())),
    }
}
