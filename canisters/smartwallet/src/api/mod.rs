mod all_addresses;
mod balance;
mod build_transaction_multisig22;
mod build_transaction_single;
mod current_fee_percentiles;
mod ecdsa_key;
mod p2pkh_address;
mod p2wsh_multisig22_address;
mod p2wsh_single_address;
mod public_key;
mod transfer_multisig22;
mod transfer_to_p2pkh;
mod utxos;

use base::domain::Wallet;
use base::tx::RawTransactionInfo;
use base::utils::{check_normal_principal, ic_caller};

use candid::Principal;
use ic_cdk::api::management_canister::bitcoin::{GetUtxosResponse, MillisatoshiPerByte, Satoshi};
use ic_cdk::{query, update};

use crate::context::{METADATA, RAW_WALLET, TRANSACTION_LOG};
use crate::domain::request::{TransferInfo, TransferRequest};
use crate::domain::response::{NetworkResponse, PublicKeyResponse};
use crate::domain::{Metadata, RawWallet, SelfCustodyKey, TransactionLog};
use crate::error::WalletError;

/// ---------------- Update interface of this canister ------------------
///
/// Returns an exists address for the caller,
/// or create a new one if it doesn't exist, and returns it
#[update]
pub async fn p2wsh_multisig22_address() -> Result<String, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;

    p2wsh_multisig22_address::serve(owner, metadata).await
}

/// Returns the single signature wallet of this canister id as diravtion path
#[update]
pub async fn p2wsh_single_address() -> Result<String, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;

    p2wsh_single_address::serve(owner, metadata).await
}

/// Returns the P2PKH address of this canister at a specific derivation path
#[update]
pub async fn p2pkh_address() -> Result<String, WalletError> {
    let owner = ic_caller();

    let metadata = validate_owner(owner)?;

    p2pkh_address::serve(owner, metadata).await
}

/// Returns the utxos of this canister address
#[update]
pub async fn utxos(address: String) -> Result<GetUtxosResponse, WalletError> {
    let network = get_metadata().network;
    utxos::serve(address, network).await
}

/// Returns all utxos of this canister's address
/// There're multiple address in a canister
/// TODO:
#[update]
pub async fn self_utxos() -> Result<Vec<GetUtxosResponse>, WalletError> {
    todo!()
}

/// Returns this canister's public key with hex string if the caller is the owner
#[update]
pub async fn public_key() -> Result<PublicKeyResponse, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;

    public_key::serve(owner, metadata).await
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
    let network = get_metadata().network;
    current_fee_percentiles::serve(network).await
}

/// Send a transaction if the caller is controller, and return txid if success
/// otherwise return `UnAuthorized`
#[update]
pub async fn transfer_single(req: TransferRequest) -> Result<String, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;

    build_transaction_single::serve(owner, metadata, req).await
}

/// Transfer btc to a p2pkh address
#[update]
pub async fn transfer_to_p2pkh(req: TransferInfo) -> Result<String, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;

    transfer_to_p2pkh::serve(owner, metadata, req.recipient, req.amount).await
}

/// Transfer btc with multisig22 wallet if the caller is owner
#[update]
pub async fn transfer_multisig22(send_request: TransferRequest) -> Result<String, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;

    transfer_multisig22::serve(owner, metadata, send_request).await
}

/// Build a transaction if the caller is controller,
/// otherwise return `UnAuthorized`
#[update]
pub async fn build_transaction_multisig22(
    req: TransferRequest,
) -> Result<RawTransactionInfo, WalletError> {
    let owner = ic_caller();
    let metadata = validate_owner(owner)?;

    build_transaction_multisig22::serve(owner, metadata, req).await
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
    get_metadata().network.into()
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
    let metadata = validate_owner(owner)?;

    Ok(all_addresses::serve(owner, metadata).await)
}

/// Validate the given ownerr if it is owner of canister, return `Metadata` if true,
/// otherwise return `UnAuthorized`
fn validate_owner(owner: Principal) -> Result<Metadata, WalletError> {
    check_normal_principal(owner)?;

    let metadata = get_metadata();
    if metadata.owner == owner {
        Ok(metadata)
    } else {
        Err(WalletError::UnAuthorized(owner.to_string()))
    }
}

fn get_metadata() -> Metadata {
    METADATA.with(|s| s.borrow().get().clone())
}

fn get_raw_wallet(key: &SelfCustodyKey) -> Option<RawWallet> {
    RAW_WALLET.with(|s| s.borrow().get(key).clone())
}

fn insert_wallet(wallet_key: SelfCustodyKey, wallet: Wallet) -> Result<(), WalletError> {
    RAW_WALLET.with(|s| {
        let mut raw_wallet = s.borrow_mut();

        match raw_wallet.get(&wallet_key) {
            Some(w) => Err(WalletError::WalletAlreadyExists(format!("{:?}", w))),
            None => {
                raw_wallet.insert(wallet_key, wallet.into());
                Ok(())
            }
        }
    })
}

fn append_transaction_log(log: TransactionLog) -> Result<(), WalletError> {
    TRANSACTION_LOG.with(|s| {
        s.borrow_mut()
            .append(&log)
            .map_err(|e| WalletError::AppendTransferLogError(format!("{:?}", e)))?;

        Ok(())
    })
}
