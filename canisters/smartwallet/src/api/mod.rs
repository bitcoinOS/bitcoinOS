mod all_addresses;
mod balance;
mod counter_increment_one;
mod current_fee_percentiles;
mod ecdsa_key;
mod get_staking;
mod list_staking;
mod logs;
mod p2pkh_address;
mod public_key;

mod register_staking;
mod staking_to_pool;
mod sync_staking_record_status;
mod total_staking;
mod transaction_log;
mod transfer_from_p2pkh;
// mod update_staking_record;
mod utxos;

use wallet::bitcoins;
use wallet::domain::request::UtxosRequest;
use wallet::domain::response::UtxosResponse;
use wallet::domain::staking::{StakingRecord, TxId};
use wallet::utils::{check_normal_principal, hex, ic_caller, ic_time, str_to_bitcoin_address};

use candid::Principal;
use ic_cdk::api::management_canister::bitcoin::{MillisatoshiPerByte, Satoshi};
use ic_cdk::{query, update};

use crate::domain::request::{
    RegisterStakingRequest, StakingRequest, TotalStakingRequest, TransferInfo, TransferRequest,
};
use crate::domain::response::{NetworkResponse, PublicKeyResponse};
use crate::domain::{Metadata, TransactionLog};
use crate::error::WalletError;
use crate::repositories::metadata::get_metadata;
use crate::repositories::{self, counter, metadata, tx_log};

/// ---------------- Update interface of this canister ------------------
///

/// Returns the P2PKH address of this canister at a specific derivation path
#[update]
pub async fn p2pkh_address() -> String {
    let metadata = get_metadata();

    p2pkh_address::serve(metadata)
        .await
        .expect("A Smart wallet must have a Bitcoin Address")
}

/// Returns the utxos of this canister default bitcoin address
#[update]
pub async fn utxos(req: UtxosRequest) -> Result<UtxosResponse, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;
    let network = metadata.network;
    let address = req.address;

    str_to_bitcoin_address(&address, network)?;

    utxos::serve(address, network).await
}

/// Returns this canister's public key with hex string if the caller is the owner
#[update]
pub async fn public_key() -> Result<PublicKeyResponse, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;

    public_key::serve(&metadata)
        .await
        .map(|res| PublicKeyResponse {
            public_key_hex: hex(res),
        })
}

/// Returns the balance of this canister default address if the caller is the owner
#[update]
pub async fn balance(address: String) -> Result<Satoshi, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;
    // let address = p2pkh_address().await;

    balance::serve(address, metadata).await
}

/// Returns the 100 fee percentiles measured in millisatoshi/byte.
/// Percentiles are computed from the last 10,000 transactions (if available).
#[update]
pub async fn current_fee_percentiles() -> Result<Vec<MillisatoshiPerByte>, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;
    let network = metadata.network;

    current_fee_percentiles::serve(network).await
}

/// Transfer btc to a p2pkh address
#[update]
pub async fn transfer_from_p2pkh(req: TransferRequest) -> Result<String, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;
    let public_key = public_key::serve(&metadata).await?;

    transfer_from_p2pkh::serve(&public_key, metadata, req).await
}

/// Staking btc to staking pool
#[update]
async fn staking_to_pool(req: StakingRequest) -> Result<String, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;
    let network = metadata.network;

    let public_key = public_key::serve(&metadata).await?;
    let sender_canister = ic_cdk::id();
    let sender_address = bitcoins::public_key_to_p2pkh_address(network, &public_key);
    let sent_time = ic_time();
    let sent_amount = req.amount;
    let staking_canister = req.staking_canister;

    let txid = staking_to_pool::serve(
        &public_key,
        metadata,
        sender_canister,
        sender_address.clone(),
        sent_time,
        req,
    )
    .await?;

    // Register staking record to staking pool
    let register_req = RegisterStakingRequest {
        txid: txid.clone(),
        sender_address,
        sent_amount,
        sent_time,
        network,
        staking_canister,
        sender: owner,
    };

    // Register staking record to staking pool cnaister
    let _record = register_staking::serve(register_req)
        .await
        .expect("Failed to register staking record");

    // TODO: Schedule a task to check the staking record status from Staking pool canister for 8 blocks by bitcoin network, and update the staking record to `Confirmed`
    // let timer_id = ic_cdk_timers::set_timer(ONE_HOURS, move || {
    //     ic_cdk::spawn(async move {
    //         sync_and_update_staking_record(record.staking_canister, record.txid.clone()).await
    //     })
    // });

    // Save timer id for upgrade canister to reschedule or cancel
    // TIMER_IDS.with_borrow_mut(|t| t.insert(timer_id, ic_time()));

    Ok(txid)
}

/// Register staking record to staking pool by manual if staking btc from a standard bitcoin wallet
#[update]
async fn register_staking(req: RegisterStakingRequest) -> Result<StakingRecord, WalletError> {
    register_staking::serve(req).await
}

/// Sync staking record status from Staking pool canister
#[update]
async fn sync_staking_record_status(txid: TxId) -> Result<bool, WalletError> {
    let owner = ic_caller();
    validate_owner(owner)?;

    sync_staking_record_status::serve(txid).await.map(|_| true)
}

/// --------------------- Queries interface of this canister -------------------
///
/// Returns the total staking amount of this canister
#[query]
fn total_staking(req: TotalStakingRequest) -> Result<Satoshi, WalletError> {
    let owner = ic_caller();
    validate_owner(owner)?;

    Ok(total_staking::serve(
        req.sender_address,
        req.staking_canister,
    ))
}

/// Returns all staking record lists of this canister
#[query]
fn list_staking() -> Result<Vec<StakingRecord>, WalletError> {
    let owner = ic_caller();
    validate_owner(owner)?;

    Ok(list_staking::serve())
}

/// Returns the staking record for the given txid
#[query]
fn get_staking(txid: TxId) -> Result<Option<StakingRecord>, WalletError> {
    let owner = ic_caller();
    validate_owner(owner)?;

    Ok(get_staking::serve(&txid))
}

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
    // validate_owner(ic_caller())
    // TODO: FIX before mainnet
    Ok(get_metadata())
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

async fn append_transaction_log(txs: &[TransferInfo]) -> Result<(), WalletError> {
    tx_log::build_and_append_transaction_log(txs)
}
