mod ecdsa_key;
mod finalize_tx_and_send;
mod public_key;

use candid::Principal;
use ic_cdk::{export_candid, init, query, update};
use wallet::domain::response::SendTransactionResponse;
use wallet::domain::EcdsaKeyIds;
use wallet::tx::RawTransactionInfo;
use wallet::utils::{principal_to_derivation_path, to_ic_bitcoin_network};

use crate::context::METADATA;
use crate::{domain::Metadata, error::StewardError};

/// --------------------- Update interface of this Canister ----------------------
///
/// Returns the public key of the given ecdsa key and caller
#[update]
pub async fn public_key() -> Vec<u8> {
    let caller = ic_caller();
    let key_id = METADATA.with(|m| m.borrow().get().ecdsa_key_id.clone());
    let derivation_path = principal_to_derivation_path(caller);

    public_key::serve(derivation_path, key_id).await.unwrap()
}

/// Finalize the tx and send it to Bitcoin network
/// Returns txid if success
///
#[update]
pub async fn finalize_tx_and_send(raw_tx_info: RawTransactionInfo) -> SendTransactionResponse {
    let wallet_canister = ic_caller();
    let metadata = METADATA.with(|m| m.borrow().get().clone());
    let network = metadata.network;
    let key_id = metadata.ecdsa_key_id;

    let txid = finalize_tx_and_send::serve(raw_tx_info, key_id, wallet_canister, network).await;

    match txid {
        Ok(txid) => SendTransactionResponse {
            txid: Some(txid),
            error_msg: None,
        },
        Err(err) => SendTransactionResponse {
            txid: None,
            error_msg: Some(err.to_string()),
        },
    }
}

/// --------------------- Queries interface of this canister -------------------
///
/// Returns ecdsa key if caller already registered
/// otherwise retunrs ECDSAKeyNotFound
#[query]
pub fn ecdsa_key() -> Result<String, StewardError> {
    ecdsa_key::serve()
}

/// Returns this canister's metadata
#[query]
fn metadata() -> Metadata {
    METADATA.with(|m| m.borrow().get().clone())
}

/// Init canister with `network` argument
#[init]
fn init(network: String) {
    METADATA.with(|m| {
        let network = to_ic_bitcoin_network(&network);
        let ecdsa_key_id = EcdsaKeyIds::from(network).to_key_id();
        let updated_time = ic_time();

        let mut metadata = m.borrow_mut();

        metadata
            .set(Metadata {
                network,
                ecdsa_key_id,
                updated_time,
            })
            .expect("Failed to init network")
    });
}

export_candid!();

fn ic_caller() -> Principal {
    ic_cdk::caller()
}

fn ic_time() -> u64 {
    ic_cdk::api::time()
}
