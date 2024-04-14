mod get_ecdsa_key;
mod public_key;

use base::domain::EcdsaKeyIds;
use base::tx::RawTransactionInfo;
use base::utils::{principal_to_derivation_path, to_ic_bitcoin_network};
use candid::Principal;
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_cdk::{export_candid, init, query, update};

use crate::context::{ECDSA_KEYS, METADATA};
use crate::domain::response::PublicKeyResponse;
use crate::{domain::Metadata, error::StewardError};

/// --------------------- Update interface of this Canister ----------------------
///
/// Returns the public key of the given ecdsa key and caller
#[update]
pub async fn public_key() -> Result<PublicKeyResponse, StewardError> {
    let caller = ic_caller();
    let key_name = METADATA.with(|m| m.borrow().get().ecdsa_key_id.name.clone());
    let derivation_path = principal_to_derivation_path(caller);

    public_key::serve(&key_name, derivation_path).await
}

/// Finalize the tx and send it to Bitcoin network
/// Returns txid if success
///
#[update]
pub async fn finalize_tx_and_send(
    network: BitcoinNetwork,
    raw_tx_info: RawTransactionInfo,
) -> Result<String, StewardError> {
    let caller = ic_caller();
    let key_name = ECDSA_KEYS
        .with(|m| m.borrow().get(&caller))
        .ok_or_else(|| StewardError::ECDSAKeyNotFound(caller.to_string()))?;

    let mut tx_info = base::tx::TransactionInfo::try_from(raw_tx_info)?;

    tx_info = base::utils::sign_transaction(
        tx_info,
        &key_name.key,
        &[caller.as_slice().to_vec()],
        base::domain::MultiSigIndex::Second,
    )
    .await?;

    base::utils::send_transaction(&tx_info, network).await?;

    Ok(tx_info.tx.txid().to_string())
}

/// --------------------- Queries interface of this canister -------------------
///
/// Returns ecdsa key if caller already registered
/// otherwise retunrs ECDSAKeyNotFound
#[query]
pub fn ecdsa_key() -> Result<String, StewardError> {
    let caller = ic_caller();
    get_ecdsa_key::serve(&caller)
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
