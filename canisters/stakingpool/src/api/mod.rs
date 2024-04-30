mod all_addresses;
mod balance;
mod logs;

mod current_fee_percentiles;
mod ecdsa_key;
mod p2pkh_address;

mod public_key;
mod transfer_from_p2pkh;
mod utxos;

use wallet::utils::{check_normal_principal, hex, ic_caller};

use candid::Principal;
use ic_cdk::api::management_canister::bitcoin::{GetUtxosResponse, MillisatoshiPerByte, Satoshi};
use ic_cdk::{query, update};

use crate::domain::request::TransferRequest;
use crate::domain::response::{NetworkResponse, PublicKeyResponse};
use crate::domain::{Metadata, TransactionLog};
use crate::error::WalletError;
use crate::repositories::{self, counter, metadata};

/// ---------------- Update interface of this canister ------------------
///

/// Returns the P2PKH address of this canister at a specific derivation path
#[update]
pub async fn p2pkh_address() -> Result<String, WalletError> {
    let owner = ic_caller();

    let metadata = validate_owner(owner)?;

    p2pkh_address::serve(metadata).await
}

/// Returns the utxos of this canister address
#[update]
pub async fn utxos(address: String) -> Result<GetUtxosResponse, WalletError> {
    let network = metadata::get_metadata().network;
    utxos::serve(address, network).await
}

/// Returns this canister's public key with hex string if the caller is the owner
#[update]
pub async fn public_key() -> Result<PublicKeyResponse, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;

    public_key::serve(metadata)
        .await
        .map(|res| PublicKeyResponse {
            public_key_hex: hex(res),
        })
}

/// Returns the balance of the given bitcoin address if the caller is the owner
#[update]
pub async fn balance(address: String) -> Result<Satoshi, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;

    balance::serve(address, metadata).await
}

/// Returns the 100 fee percentiles measured in millisatoshi/byte.
/// Percentiles are computed from the last 10,000 transactions (if available).
#[update]
pub async fn current_fee_percentiles() -> Result<Vec<MillisatoshiPerByte>, WalletError> {
    let network = metadata::get_metadata().network;
    current_fee_percentiles::serve(network).await
}

/// Transfer btc to a p2pkh address
#[update]
pub async fn transfer_from_p2pkh(req: TransferRequest) -> Result<String, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;

    transfer_from_p2pkh::serve(metadata, req).await
}

/// --------------------- Queries interface of this canister -------------------
///
/// Returns ecdsa key of this canister if the caller is controller and the key exists
/// otherwise return `EcdsaKeyNotFound` or `UnAuthorized`
#[query]
pub fn ecdsa_key() -> Result<String, WalletError> {
    let owner = ic_caller();
    ecdsa_key::serve(owner)
}

/// Returns the network of this canister
#[query]
fn network() -> NetworkResponse {
    metadata::get_metadata().network.into()
}

/// Returns the metadata of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
fn metadata() -> Result<Metadata, WalletError> {
    validate_owner(ic_caller())
    // Ok(get_metadata())
}

/// Returns the owner of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
fn owner() -> Result<Principal, WalletError> {
    let owner = ic_caller();

    validate_owner(owner).map(|m| m.owner)
}

/// Returns all the addresses of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
async fn addresses() -> Result<Vec<String>, WalletError> {
    let owner = ic_caller();
    let _metadata = validate_owner(owner)?;

    Ok(all_addresses::serve().await)
}

/// Returns all ledger records of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
async fn logs() -> Result<Vec<TransactionLog>, WalletError> {
    let owner = ic_caller();
    let _metadata = validate_owner(owner)?;

    Ok(logs::serve().await)
}

/// Returns the counter of this canister
#[query]
fn counter() -> u128 {
    counter::get_counter()
}

/// Validate the given ownerr if it is owner of canister, return `Metadata` if true,
/// otherwise return `UnAuthorized`
fn validate_owner(owner: Principal) -> Result<Metadata, WalletError> {
    check_normal_principal(owner)?;

    let metadata = repositories::metadata::get_metadata();
    if metadata.owner == owner {
        Ok(metadata)
    } else {
        Err(WalletError::UnAuthorized(owner.to_string()))
    }
}
