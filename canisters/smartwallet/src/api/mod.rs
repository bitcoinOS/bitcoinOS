mod balance;
mod build_transaction_with_multisig22_p2wsh;
mod build_transaction_with_single_p2wsh;
mod current_fee_percentiles;
mod ecdsa_key;
mod get_or_create_multisig22_wallet_address;
mod get_or_create_single_p2wsh_wallet;
mod p2pkh_address;
mod public_key;
mod utxos;

use base::domain::Wallet;
use base::tx::RawTransactionInfo;
use base::utils::{ic_caller, principal_to_derivation_path};

use candid::Principal;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::api::management_canister::bitcoin::{GetUtxosResponse, MillisatoshiPerByte, Satoshi};
use ic_cdk::{query, update};

use crate::context::{METADATA, RAW_WALLET, TRANSACTION_LOG};
use crate::domain::request::TransferRequest;
use crate::domain::response::{NetworkResponse, PublicKeyResponse};
use crate::domain::{Metadata, RawWallet, SelfCustodyKey, TransactionLog};
use crate::error::WalletError;

/// ---------------- Update interface of this canister ------------------
///
/// Returns an exists address for the caller,
/// or create a new one if it doesn't exist, and returns it
#[update]
pub async fn get_or_create_multisig22_wallet_p2wsh_address() -> Result<String, WalletError> {
    let caller = ic_caller();

    get_or_create_multisig22_wallet_address::serve(caller).await
}

/// Returns the single signature wallet of this canister id as diravtion path
#[update]
pub async fn get_or_create_single_wallet_p2wsh_address() -> Result<String, WalletError> {
    let caller = ic_caller();
    let metadata = get_metadata();
    let key_id = metadata.ecdsa_key_id;
    let steward_canister = metadata.steward_canister;
    let network = metadata.network;

    get_or_create_single_p2wsh_wallet::serve(caller, key_id, steward_canister, network).await
}

/// Returns the P2PKH address of this canister at a specific derivation path
#[update]
pub async fn p2pkh_address() -> Result<String, WalletError> {
    let caller = ic_caller();
    let derivation_path = principal_to_derivation_path(caller);
    let metadata = get_metadata();

    let key_id = metadata.ecdsa_key_id;
    let network = metadata.network;

    p2pkh_address::serve(network, derivation_path, key_id).await
}

/// Returns the utxos of this canister address if the caller is controller
#[update]
pub async fn utxos(address: String) -> Result<GetUtxosResponse, WalletError> {
    let network = get_metadata().network;
    utxos::serve(address, network).await
}

/// Returns this canister's public key with hex string
#[update]
pub async fn public_key() -> Result<PublicKeyResponse, WalletError> {
    let caller = ic_caller();
    let key_id = get_metadata().ecdsa_key_id;
    let derivation_path = principal_to_derivation_path(caller);

    public_key::serve(derivation_path, key_id).await
}

/// Returns the balance of the given bitcoin address
#[update]
pub async fn balance(address: String) -> Result<Satoshi, WalletError> {
    let caller = ic_caller();
    balance::serve(address, caller).await
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
    let caller = ic_caller();

    build_transaction_with_single_p2wsh::serve(caller, req).await
}

#[update]
pub async fn transfer_multisig22(send_request: TransferRequest) -> Result<String, WalletError> {
    let tx_info = build_transaction_multisig22(send_request).await.unwrap();
    let metadata = get_metadata();
    let key_id = metadata.ecdsa_key_id;
    let network = metadata.network;
    let steward_caninster = metadata.steward_canister;
    let wallet_canister = ic_cdk::id();

    ic_cdk::print(format!("{:?}", tx_info));

    let resp: Result<(String,), (RejectionCode, String)> = ic_cdk::api::call::call(
        steward_caninster,
        "finalize_tx_and_send",
        (tx_info, key_id, wallet_canister, network),
    )
    .await;

    resp.map(|(txid,)| txid)
        .map_err(|e| WalletError::StewardCallError(format!("{:?}", e)))
}

/// Build a transaction if the caller is controller,
/// otherwise return `UnAuthorized`
#[update]
pub async fn build_transaction_multisig22(
    req: TransferRequest,
) -> Result<RawTransactionInfo, WalletError> {
    let caller = ic_caller();

    build_transaction_with_multisig22_p2wsh::serve(caller, req).await
}

/// --------------------- Queries interface of this canister -------------------
///
/// Returns ecdsa key of this canister if the caller is controller and the key exists
/// otherwise return `EcdsaKeyNotFound` or `UnAuthorized`
#[query]
pub fn ecdsa_key() -> Result<String, WalletError> {
    let caller = ic_caller();
    ecdsa_key::serve(caller)
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
    // validate_owner(ic_caller())
    Ok(get_metadata())
}

/// Returns the owner of this canister if the caller is controller
/// otherwise return `UnAuthorized`
#[query]
fn owner() -> Result<Principal, WalletError> {
    let caller = ic_caller();

    validate_owner(caller).map(|m| m.owner)
}

/// Validate the caller if it is owner, return `Metadata` if true,
/// otherwise return `UnAuthorized`
fn validate_owner(caller: Principal) -> Result<Metadata, WalletError> {
    let metadata = get_metadata();
    if metadata.owner == caller {
        Ok(metadata)
    } else {
        Err(WalletError::UnAuthorized(caller.to_string()))
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
