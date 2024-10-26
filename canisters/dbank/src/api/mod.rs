mod all_addresses;
mod balance_of;
mod count_wallet;
mod counter_increment_one;
mod create_p2pkh_wallet;
mod current_fee_percentiles;
mod ecdsa_key;

mod list_wallet;

mod create_p2wpkh_wallet;
mod p2pkh_address;
mod p2wpkh_address;
mod public_key;

mod set_steward_canister;
mod staking_to_pool;
mod staking_to_pool_from_p2wpkh;

mod transfer_from_p2pkh;
mod transfer_from_p2wpkh;

mod tx_log_of;
mod tx_logs;

mod utxos;

mod wallet_log_of;
mod wallet_logs;

use ic_cdk::api::is_controller;
use ic_cdk::api::management_canister::main::CanisterId;

use wallet::domain::request::{StakingRequest, TransferRequest, UtxosRequest};
use wallet::domain::response::UtxosResponse;
use wallet::tx::RecipientAmount;
use wallet::utils::{check_normal_principal, hex, ic_caller, str_to_bitcoin_address};

use candid::Principal;
use ic_cdk::api::management_canister::bitcoin::{MillisatoshiPerByte, Satoshi};
use ic_cdk::{query, update};

use crate::domain::request::CreateDBankWalletRequest;
use crate::domain::response::{NetworkResponse, PublicKeyResponse};
use crate::domain::{DBankWalletInfo, Metadata, TransactionLog, WalletOperationEvent};
use crate::error::DBankError;
use crate::repositories::metadata::get_metadata;
use crate::repositories::{self, metadata, sequencer};

/// ---------------- Update interface of this canister ------------------
///

/// Returns the bitcoin P2PKH address of the caller
#[query]
pub fn p2pkh_address() -> Option<String> {
    let wallet_owner = ic_caller();
    let metadata = get_metadata();
    p2pkh_address::serve(&metadata, wallet_owner)
}

/// Create a bitcoin P2PKH address's wallet.
/// Returns a P2PKH address of this canister at a specific derivation path
#[update]
pub async fn create_p2pkh_wallet(req: CreateDBankWalletRequest) -> String {
    let dbank_owner = ic_caller();

    let metadata = validate_owner(dbank_owner).expect("Only owner can generate P2PKH address");

    create_p2pkh_wallet::serve(req.seq_in_os, metadata, req.wallet_owner, req.name)
        .await
        .expect("A Smart wallet must have a Bitcoin Address")
}

/// Returns a P2WPKH address of this canister at a specific derivation path
#[query]
pub fn p2wpkh_address() -> Option<String> {
    let wallet_owner = ic_caller();
    let metadata = get_metadata();
    p2wpkh_address::serve(&metadata, wallet_owner)
}

/// Returns the P2WSH address of this canister at a specific derivation path
#[query]
pub async fn p2wsh_multisig22_address() -> String {
    p2wpkh_address().expect("Can't find a bitcoin address for caller")
}

/// Create a bitcoin p2wpkh address's wallet
/// Returns a P2WPKH address of this canister at a specific derivation path
#[update]
pub async fn create_p2wpkh_wallet(req: CreateDBankWalletRequest) -> String {
    let controller = ic_caller();

    let metadata = validate_owner(controller).expect("Only owner can generate P2WPKH address");

    create_p2wpkh_wallet::serve(req.seq_in_os, metadata, req.wallet_owner, req.name)
        .await
        .expect("A Smart wallet must have a Bitcoin Address")
}

/// Returns the utxos for given bitcoin address, only available for the wallet owner
#[update]
pub async fn utxos(req: UtxosRequest) -> Result<UtxosResponse, DBankError> {
    let owner = ic_caller();

    validate_wallet_owner(owner)?;

    let metadata = get_metadata();

    let network = metadata.network;
    let address = req.address;

    str_to_bitcoin_address(&address, network)?;

    utxos::serve(address, network).await
}

/// Returns the balance of this canister default address if the caller is the owner
#[update]
pub async fn balance(address: String) -> Result<Satoshi, DBankError> {
    let owner = ic_caller();

    validate_wallet_owner(owner)?;

    let metadata = get_metadata();

    balance_of::serve(address, metadata).await
}

/// Returns the 100 fee percentiles measured in millisatoshi/byte.
/// Percentiles are computed from the last 10,000 transactions (if available).
#[update]
pub async fn current_fee_percentiles() -> Result<Vec<MillisatoshiPerByte>, DBankError> {
    let owner = ic_caller();

    validate_wallet_owner(owner)?;

    let network = get_metadata().network;

    current_fee_percentiles::serve(network).await
}

/// Transfer btc from a p2pkh wallet
#[update]
pub async fn transfer_from_p2pkh(req: TransferRequest) -> Result<String, DBankError> {
    let owner = ic_caller();
    validate_wallet_owner(owner)?;

    let metadata = get_metadata();
    let public_key = public_key::serve(owner, &metadata)?;

    let txs = req.validate_address(metadata.network)?;
    transfer_from_p2pkh::serve(&public_key, owner, metadata, &txs.txs).await
}

/// Transfer btc to a p2wpkh address
#[update]
pub async fn transfer_from_p2wpkh(req: TransferRequest) -> Result<String, DBankError> {
    let owner = ic_caller();
    validate_wallet_owner(owner)?;

    let metadata = get_metadata();

    let public_key = public_key::serve(owner, &metadata)?;

    let txs = req.validate_address(metadata.network)?;
    transfer_from_p2wpkh::serve(&public_key, owner, metadata, &txs.txs).await
}

/// Transfer btc from a p2wpkh address
/// Note: The name of this interface is to be compatible with the interface of the SMARTWALLET canister
#[update]
pub async fn transfer_from_p2wsh_multisig22(req: TransferRequest) -> Result<String, DBankError> {
    transfer_from_p2wpkh(req).await
}

/// Staking btc to staking pool
#[update]
async fn staking_to_pool(req: StakingRequest) -> Result<String, DBankError> {
    let owner = ic_caller();

    validate_wallet_owner(owner)?;

    let metadata = get_metadata();

    let public_key = public_key::serve(owner, &metadata)?;

    let txid = staking_to_pool::serve(&public_key, owner, metadata, req).await?;

    Ok(txid)
}

/// Staking btc to staking pool
#[update]
async fn staking_to_pool_from_p2wpkh(req: StakingRequest) -> Result<String, DBankError> {
    let owner = ic_caller();

    validate_wallet_owner(owner)?;

    let metadata = get_metadata();

    let public_key = public_key::serve(owner, &metadata)?;

    let txid = staking_to_pool_from_p2wpkh::serve(&public_key, owner, metadata, req).await?;

    Ok(txid)
}

/// Staking btc to staking pool
/// Note: The name of this interface is to be compatible with the interface of the SMARTWALLET canister
#[update]
async fn staking_to_pool_from_p2wsh_multisig22(req: StakingRequest) -> Result<String, DBankError> {
    staking_to_pool_from_p2wpkh(req).await
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
/// Returns this canister's public key with hex string for given wallet_owner
#[query]
pub async fn public_key(wallet_owner: Principal) -> Result<PublicKeyResponse, DBankError> {
    let metadata = get_metadata();
    public_key::serve(wallet_owner, &metadata).map(|res| PublicKeyResponse {
        public_key_hex: hex(res),
    })
}

/// Returns ecdsa key of this canister if the caller is controller and the key exists
/// otherwise return `EcdsaKeyNotFound` or `UnAuthorized`
#[query]
pub fn ecdsa_key() -> String {
    ecdsa_key::serve()
}

/// Returns the network of this canister
#[query]
fn network() -> NetworkResponse {
    metadata::get_metadata().network.into()
}

/// Returns the metadata of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
fn metadata() -> Result<Metadata, DBankError> {
    // validate_owner(ic_caller())
    // TODO: FIX before mainnet
    Ok(get_metadata())
}

/// Returns all wallets of this canister
#[update]
fn list_wallet() -> Vec<DBankWalletInfo> {
    // let owner = ic_caller();

    // if validate_owner(owner).is_err() {
    //     return vec![];
    // }

    list_wallet::serve()
}

/// Returns all the addresses of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
async fn addresses() -> Result<Vec<String>, DBankError> {
    // let owner = ic_caller();
    // let _metadata = validate_owner(owner)?;

    Ok(all_addresses::serve().await)
}

/// Returns a transaction log for the given index
#[query]
fn tx_log_of_index(index: u64) -> Option<TransactionLog> {
    tx_log_of::serve(index)
}

/// Returns all transactions of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
async fn tx_logs() -> Result<Vec<TransactionLog>, DBankError> {
    let owner = ic_caller();
    let _metadata = validate_owner(owner)?;

    Ok(tx_logs::serve().await)
}

/// Returns all wallet operations of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
async fn wallet_logs() -> Result<Vec<WalletOperationEvent>, DBankError> {
    // let owner = ic_caller();
    // let _metadata = validate_owner(owner)?;

    Ok(wallet_logs::serve().await)
}

/// Returns a transaction log for the given index
#[query]
fn wallet_log_of_index(index: u64) -> Option<WalletOperationEvent> {
    wallet_log_of::serve(index)
}

/// Returns the counter of this canister
#[query]
fn counter() -> u64 {
    sequencer::get_sequencer()
}

#[query]
fn count_wallet() -> u64 {
    count_wallet::serve()
}

/// Validate the given ownerr if it is owner of canister, return `Metadata` if true,
/// otherwise return `UnAuthorized`
fn validate_owner(owner: Principal) -> Result<Metadata, DBankError> {
    check_normal_principal(owner)?;

    let metadata = repositories::metadata::get_metadata();
    if metadata.owner == owner || is_controller(&owner) {
        Ok(metadata)
    } else {
        Err(DBankError::UnAuthorized(owner.to_string()))
    }
}

/// Validate caller is wallet owner or not
fn validate_wallet_owner(caller: Principal) -> Result<(), DBankError> {
    check_normal_principal(caller)?;

    // if repositories::wallet_info::contains(&caller) {
    //     Ok(())
    // } else {
    //     Err(DBankError::UnAuthorized(caller.to_string()))
    // }
    Ok(())
}

async fn append_transaction_log(txs: &[RecipientAmount]) -> Result<(), DBankError> {
    repositories::tx_logs::build_and_append_transaction_log(txs)
}
