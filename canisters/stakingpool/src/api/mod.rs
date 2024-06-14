mod balance;
mod confirm_staking_record;
mod confirm_staking_record_one;
mod get_staking;
mod list_staking;
mod logs;
mod p2pkh_address;
mod p2wsh_multisig22_address;
mod public_key;
mod redeem;

mod register_staking;
mod set_steward_canister;
mod staker_save;
mod transfer_from_p2pkh;
mod transfer_from_p2wsh_multisig22;
mod tvl;
mod utxos;

use candid::Principal;
use ic_cdk::api::is_controller;
use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, Satoshi};
use ic_cdk::api::management_canister::main::CanisterId;
use ic_cdk::{query, update};
use wallet::domain::request::{TransferRequest, UtxosRequest};
use wallet::domain::response::UtxosResponse;
use wallet::domain::staking::StakingRecord;
use wallet::domain::TxId;
use wallet::utils::{check_normal_principal, ic_caller, ic_time, str_to_bitcoin_address};

use crate::domain::request::{RedeemRequest, RegisterStakingRequest};
use crate::domain::response::NetworkResponse;
use crate::domain::{Metadata, RedeemLog};
use crate::error::StakingError;
use crate::repositories;

/// ---------------- Update interface of this canister ------------------
///

/// Returns the P2PKH address of this staking pool canister
#[update]
pub async fn p2pkh_address() -> String {
    let metadata = get_metadata();

    p2pkh_address::serve(metadata)
        .await
        .expect("Staking pool must has a bitcoin address")
}

/// Returns the P2WSH address of this canister at a specific derivation path
#[update]
pub async fn p2wsh_multisig22_address() -> String {
    let metadata = get_metadata();

    p2wsh_multisig22_address::serve(metadata)
        .await
        .expect("A Smart wallet must have a Bitcoin Address")
}

/// Transfer btc from a p2pkh wallet
#[update]
pub async fn transfer_from_p2pkh(req: TransferRequest) -> Result<String, StakingError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;
    let public_key = public_key::serve(&metadata).await?;

    transfer_from_p2pkh::serve(&public_key, metadata, req).await
}

/// Transfer btc to a ppkh address
#[update]
pub async fn transfer_from_p2wsh_multisig22(req: TransferRequest) -> Result<String, StakingError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;
    // let public_key = public_key::serve(&metadata).await?;

    transfer_from_p2wsh_multisig22::serve(metadata, req).await
}
/// Returns the utxos of this staking pool canister
#[update]
pub async fn utxos(req: UtxosRequest) -> Result<UtxosResponse, StakingError> {
    let metadata = get_metadata();
    let network = metadata.network;
    let address = req.address;

    str_to_bitcoin_address(&address, network)?;

    utxos::serve(address, network, req.filter).await
}

/// Returns the balance of this staking pool
#[update]
pub async fn balance(address: String) -> Result<Satoshi, StakingError> {
    let metadata = get_metadata();
    let network = metadata.network;

    balance::serve(address, network).await
}

/// Register a new staking record after smartwallet staked btc to pools
/// Returns the staking record
/// NOTE: If the amount of staking record data is too large, it can be migrated to a dedicated data canister cluster.
#[update]
async fn register_staking_record(req: RegisterStakingRequest) -> StakingRecord {
    let sender_canister = ic_caller();
    check_normal_principal(sender_canister).expect("caller is not normal principal");

    check_network(req.network).expect("invalid network");

    let staking_canister = ic_cdk::id();
    let staking_address = p2pkh_address::serve(get_metadata())
        .await
        .expect("Staking pool must has a bitcoin address");

    let updated_time = ic_time();
    let metadata = get_metadata();
    let duration = metadata.duration_in_day;
    let interest_rate = metadata.annual_interest_rate;

    staker_save::serve(sender_canister, updated_time);

    register_staking::serve(
        sender_canister,
        updated_time,
        req,
        interest_rate,
        duration,
        staking_canister,
        staking_address,
    )
    .expect("Failed to register staking record")

    // TODO: Schedule a task to check the txid confirmed for 6 blocks by bitcoin network, and update the staking record to `Confirmed`
}

/// Sync all `Pending` staking record to `Confirmed`
#[update]
async fn confirm_staking_record() -> bool {
    let caller = ic_caller();

    if !is_controller(&caller) {
        return false;
    }

    let metadata = get_metadata();
    confirm_staking_record::serve(metadata).await.is_ok()
}

/// Sysnc a staking record `Pending` to `Confirmed` for a given txid
#[update]
async fn confirm_staking_record_one(txid: TxId) -> Option<StakingRecord> {
    let caller = ic_caller();

    let metadata = get_metadata();

    confirm_staking_record_one::serve(caller, txid, metadata)
        .await
        .expect("Failed to confirm staking record")
}

/// Redeem btc from this canister, and return the txid,
/// When user redeems, it will redeems the amount that is received amount + interest
/// NOTE: Must validate the staker and amount is valid
/// NOTE: Only staker canister can redeem now, this will change to wrapper token in the future
/// NOTE: After osBTC issued, this will change
#[update]
pub async fn redeem(req: RedeemRequest) -> Result<String, StakingError> {
    check_network(req.network)?;

    let metadata = get_metadata();
    let sender = ic_cdk::caller();
    let redeem_time = ic_time();

    redeem::serve(sender, metadata, req, redeem_time).await
}

/// Update the steward canister id
#[ic_cdk::update]
fn set_steward_canister(canister_id: CanisterId) -> String {
    if is_controller(&ic_cdk::caller()) {
        set_steward_canister::serve(canister_id)
    } else {
        "UnAuthorized".to_string()
    }
}

/// --------------------- Queries interface of this canister -------------------
///
/// Returns TVL of this staking pool canister
#[query]
async fn tvl() -> Satoshi {
    // balance().await.unwrap()
    tvl::serve()
}

/// Returns all staking record lists of this canister
#[query]
fn list_staking() -> Result<Vec<StakingRecord>, StakingError> {
    Ok(list_staking::serve())
}

/// Query a staking record of given txid
/// Returns Some(StakingRecord) if found, otherwise return None
#[query]
fn get_staking(txid: String) -> Option<StakingRecord> {
    get_staking::serve(txid)
}

/// Returns the network of this canister
#[query]
fn network() -> NetworkResponse {
    get_metadata().network.into()
}

/// Returns the metadata of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
fn metadata() -> Result<Metadata, StakingError> {
    Ok(get_metadata())
}

/// Returns all ledger records of this canister if the caller is controller
/// otherwise return `UnAuthorized`
/// TODO: FIX large data, Paginate will need
#[query]
async fn redeem_logs() -> Vec<RedeemLog> {
    logs::serve().await
}

/// Returns the counter of this canister
#[query]
fn counter() -> u128 {
    get_counter()
}

/// Helpers functions

/// Validate the given ownerr if it is owner of canister, return `Metadata` if true,
/// otherwise return `UnAuthorized`
fn validate_owner(owner: Principal) -> Result<Metadata, StakingError> {
    check_normal_principal(owner)?;

    let metadata = repositories::metadata::get_metadata();
    if metadata.owner == owner || is_controller(&owner) {
        Ok(metadata)
    } else {
        Err(StakingError::UnAuthorized(owner.to_string()))
    }
}

fn check_network(network: BitcoinNetwork) -> Result<(), StakingError> {
    let current_network = get_metadata().network;

    if current_network == network {
        Ok(())
    } else {
        Err(StakingError::InvalidNetwork)
    }
}

fn get_metadata() -> Metadata {
    repositories::metadata::get_metadata()
}

fn get_counter() -> u128 {
    repositories::counter::get_counter()
}
