mod balance;
mod logs;
mod p2pkh_address;
mod public_key;
mod redeem;
mod utxos;

use ic_cdk::api::management_canister::bitcoin::{GetUtxosResponse, Satoshi, UtxoFilter};
use ic_cdk::{query, update};

use crate::domain::request::RedeemRequest;
use crate::domain::response::NetworkResponse;
use crate::domain::{Metadata, RedeemLog};
use crate::error::WalletError;
use crate::repositories::{self, counter, metadata};

/// ---------------- Update interface of this canister ------------------
///

/// Returns the P2PKH address of this canister at a specific derivation path
#[update]
pub async fn p2pkh_address() -> Result<String, WalletError> {
    let metadata = repositories::metadata::get_metadata();

    p2pkh_address::serve(metadata).await
}

/// Returns the utxos of this canister address
#[update]
pub async fn utxos(filter: Option<UtxoFilter>) -> Result<GetUtxosResponse, WalletError> {
    let metadata = metadata::get_metadata();
    let network = metadata.network;
    let address = p2pkh_address::serve(metadata).await?;

    utxos::serve(address, network, filter).await
}

/// Returns the balance of the given bitcoin address if the caller is the owner
#[update]
pub async fn balance() -> Result<Satoshi, WalletError> {
    let metadata = repositories::metadata::get_metadata();
    let network = metadata.network;
    let address = p2pkh_address::serve(metadata).await?;
    balance::serve(address, network).await
}

/// Transfer btc to a p2pkh address
#[update]
pub async fn redeem(req: RedeemRequest) -> Result<String, WalletError> {
    let metadata = repositories::metadata::get_metadata();

    redeem::serve(metadata, req).await
}

/// --------------------- Queries interface of this canister -------------------
///
/// Returns the network of this canister
#[query]
fn network() -> NetworkResponse {
    metadata::get_metadata().network.into()
}

/// Returns the metadata of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
fn metadata() -> Result<Metadata, WalletError> {
    Ok(repositories::metadata::get_metadata())
}

/// Returns all ledger records of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
async fn logs() -> Result<Vec<RedeemLog>, WalletError> {
    Ok(logs::serve().await)
}

/// Returns the counter of this canister
#[query]
fn counter() -> u128 {
    counter::get_counter()
}
