mod balance;
mod build_transaction;
mod ecdsa_key;
mod public_key;

use base::tx::RawTransactionInfo;
use base::utils::{ic_caller, principal_to_derivation_path};
use candid::Principal;
use ic_cdk::api::management_canister::bitcoin::Satoshi;
use ic_cdk::{query, update};

use crate::context::{State, STATE};
use crate::domain::request::TransferRequest;
use crate::domain::response::{NetworkResponse, PublicKeyResponse};
use crate::domain::Metadata;
use crate::error::WalletError;

/// ---------------- Update interface of this canister ------------------
///
/// Returns an exists address for the caller,
/// or create a new one if it doesn't exist, and returns it
#[update]
pub async fn get_or_create_wallet_address() -> Result<String, WalletError> {
    let caller = ic_caller();

    crate::bitcoin::get_or_create_wallet_address(caller).await
}

/// Returns this canister's public key with hex string
#[update]
pub async fn public_key() -> Result<PublicKeyResponse, WalletError> {
    let caller = ic_caller();
    let key_name = STATE.with(|s| s.borrow().metadata.get().ecdsa_key_id.name.clone());
    let derivation_path = principal_to_derivation_path(caller);

    public_key::serve(&key_name, derivation_path).await
}

/// Returns the balance of the given bitcoin address
#[update]
pub async fn balance(address: String) -> Result<Satoshi, WalletError> {
    let caller = ic_caller();
    balance::serve(address, caller).await
}

/// Build a transaction if the caller is controller,
/// otherwise return `UnAuthorized`
#[update]
pub async fn build_transaction(req: TransferRequest) -> Result<RawTransactionInfo, WalletError> {
    let caller = ic_caller();

    build_transaction::serve(caller, req).await
}

/// --------------------- Queries interface of this canister -------------------
///
/// Returns ecdsa key of this canister if the caller is controller and the key exists
/// otherwise return `EcdsaKeyNotFound` or `UnAuthorized`
#[query]
pub fn ecdsa_key() -> Result<String, WalletError> {
    let caller = ic_caller();
    ecdsa_key::serve(&caller)
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

/// Returns the owner of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
fn owner() -> Result<Principal, WalletError> {
    let caller = ic_caller();

    STATE.with(|s| {
        let state = s.borrow();

        validate_controller(&state, &caller, |s| Ok(s.metadata.get().owner))
    })
}

fn validate_controller<F, T>(state: &State, caller: &Principal, f: F) -> Result<T, WalletError>
where
    F: FnOnce(&State) -> Result<T, WalletError>,
{
    if state.metadata.get().owner == *caller {
        f(state)
    } else {
        Err(WalletError::UnAuthorized(caller.to_string()))
    }
}

#[allow(unused)]
fn validate_controller_mut<F, T>(
    state: &mut State,
    caller: &Principal,
    mut f: F,
) -> Result<T, WalletError>
where
    F: FnMut(&mut State) -> Result<T, WalletError>,
{
    if state.metadata.get().owner == *caller {
        f(state)
    } else {
        Err(WalletError::UnAuthorized(caller.to_string()))
    }
}
