use bitcoin::consensus;

use bitcoin::Transaction;
use bitcoin::Txid;

use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use wallet::bitcoins;
use wallet::tx::RecipientAmount;
use wallet::utils::{self, principal_to_derivation_path};
use wallet::{constants::DEFAULT_FEE_MILLI_SATOSHI, utils::str_to_bitcoin_address};

use crate::domain::request::RedeemRequest;
use crate::domain::Metadata;
use crate::error::WalletError;
use crate::repositories::counter;
use crate::repositories::tx_log;

use super::public_key;

pub(super) async fn serve(metadata: Metadata, req: RedeemRequest) -> Result<String, WalletError> {
    let tx = req.validate_address(metadata.network)?;

    // Log transfer info
    tx_log::build_and_append_redeem_log(req)?;

    // Transaction counter increment one
    counter::increment_one();

    send_p2pkh_transaction(metadata, tx)
        .await
        .map(|txid| txid.to_string())
}

/// Send a transaction to bitcoin network that transfer the given amount and recipient
/// and the sender is the canister itself
pub async fn send_p2pkh_transaction(
    metadata: Metadata,
    tx: RecipientAmount,
) -> Result<Txid, WalletError> {
    let network = metadata.network;
    let key_id = metadata.ecdsa_key_id.clone();
    let derivation_path = principal_to_derivation_path(metadata.owner);

    // Get fee per byte
    let fee_per_byte = bitcoins::get_fee_per_byte(network, DEFAULT_FEE_MILLI_SATOSHI).await?;

    // Fetch public key, p2pkh address, and utxos
    let sender_public_key = public_key::serve(metadata).await?;

    let sender_address = bitcoins::public_key_to_p2pkh_address(network, &sender_public_key);
    let sender_address = str_to_bitcoin_address(&sender_address, network)?;

    ic_cdk::print(format!(
        "Sender address: {sender_address:?} ---------------------- \n"
    ));

    // Fetching UTXOs
    ic_cdk::print("Fetching UTXOs... \n");

    // TODO: UTXOs maybe very large, need to paginate
    let utxos = wallet::bitcoins::get_utxos(sender_address.to_string(), network, None)
        .await?
        .utxos;

    // Build transaction
    let tx = utils::build_transaction(
        &sender_public_key,
        &sender_address,
        &utxos,
        &[tx],
        fee_per_byte,
    )
    .await?;

    // Sign the transaction
    let signed_tx = utils::sign_transaction_p2pkh(
        &sender_public_key,
        &sender_address,
        tx,
        key_id,
        derivation_path,
        wallet::ecdsa::sign_with_ecdsa_uncheck,
    )
    .await?;

    send_transaction(&signed_tx, network).await
}

async fn send_transaction(tx: &Transaction, network: BitcoinNetwork) -> Result<Txid, WalletError> {
    let signed_tx_bytes = consensus::serialize(tx);
    ic_cdk::print(format!("Signed tx: {:?} \n", hex::encode(&signed_tx_bytes)));

    let txid = tx.compute_txid();

    ic_cdk::print(format!("Sending transaction... {txid:?}\n"));

    wallet::bitcoins::send_transaction(signed_tx_bytes, network).await?;

    ic_cdk::print("Transaction sent! \n");

    Ok(txid)
}
