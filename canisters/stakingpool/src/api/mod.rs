mod balance;
mod logs;
mod p2pkh_address;
mod public_key;
mod redeem;
mod register_staking;
mod utxos;

use ic_cdk::api::management_canister::bitcoin::{
    BitcoinNetwork, GetUtxosResponse, Satoshi, UtxoFilter,
};
use ic_cdk::{query, update};
use wallet::utils::{check_normal_principal, ic_caller, ic_time};

use crate::domain::request::{RedeemRequest, RegisterStakingRequest};
use crate::domain::response::NetworkResponse;
use crate::domain::{Metadata, RedeemLog, StakingRecord};
use crate::error::StakingError;
use crate::repositories::{self, counter, metadata};

/// ---------------- Update interface of this canister ------------------
///

/// Returns the P2PKH address of this staking pool canister
#[update]
pub async fn p2pkh_address() -> String {
    let metadata = repositories::metadata::get_metadata();

    p2pkh_address::serve(metadata)
        .await
        .expect("Staking pool must has a bitcoin address")
}

/// Returns the utxos of this staking pool canister
#[update]
pub async fn utxos(filter: Option<UtxoFilter>) -> Result<GetUtxosResponse, StakingError> {
    let metadata = metadata::get_metadata();
    let network = metadata.network;
    let address = p2pkh_address::serve(metadata).await?;

    utxos::serve(address, network, filter).await
}

/// Returns the balance of this staking pool
#[update]
pub async fn balance() -> Result<Satoshi, StakingError> {
    let metadata = repositories::metadata::get_metadata();
    let network = metadata.network;
    let address = p2pkh_address::serve(metadata).await?;
    balance::serve(address, network).await
}

/// Register a new staking record after smartwallet staked btc to pools
/// Returns the staking record
/// NOTE: If the amount of staking record data is too large, it can be migrated to a dedicated data canister cluster.
#[update]
async fn register_staking_record(
    req: RegisterStakingRequest,
) -> Result<StakingRecord, StakingError> {
    let sender = ic_caller();
    check_normal_principal(sender)?;

    check_network(req.network)?;

    let updated_time = ic_time();
    let duration = repositories::metadata::get_metadata().duration_in_millisecond;

    register_staking::serve(sender, updated_time, req, duration).await

    // TODO: Schedule a task to check the txid confirmed for 6 blocks by bitcoin network, and update the staking record to `Confirmed`
}

/// Redeem btc from this canister, and return the txid,
/// When user redeems, it will redeems the amount that is received amount + interest
/// NOTE: Must validate the staker and amount is valid
/// NOTE: Only staker canister can redeem now, this will change to wrapper token in the future
#[update]
pub async fn redeem(req: RedeemRequest) -> Result<String, StakingError> {
    check_network(req.network)?;

    let metadata = repositories::metadata::get_metadata();
    let sender = ic_cdk::caller();
    let redeem_time = ic_time();

    redeem::serve(sender, metadata, req, redeem_time).await
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
fn metadata() -> Result<Metadata, StakingError> {
    Ok(repositories::metadata::get_metadata())
}

/// Returns all ledger records of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
async fn logs() -> Result<Vec<RedeemLog>, StakingError> {
    Ok(logs::serve().await)
}

/// Returns the counter of this canister
#[query]
fn counter() -> u128 {
    counter::get_counter()
}

fn check_network(network: BitcoinNetwork) -> Result<(), StakingError> {
    let current_network = repositories::metadata::get_metadata().network;

    if current_network == network {
        Ok(())
    } else {
        Err(StakingError::InvalidNetwork)
    }
}
