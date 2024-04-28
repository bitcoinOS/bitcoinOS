use bitcoin::consensus;
use bitcoin::Txid;
use candid::Principal;
use ic_cdk::api::management_canister::{
    bitcoin::{
        bitcoin_get_current_fee_percentiles, BitcoinNetwork, GetCurrentFeePercentilesRequest,
    },
    ecdsa::EcdsaKeyId,
};
use wallet::tx::RecipientAmount;
use wallet::utils::{self, principal_to_derivation_path};
use wallet::{constants::DEFAULT_FEE_MILLI_SATOSHI, utils::str_to_bitcoin_address};

use crate::domain::request::TransferRequest;
use crate::domain::Metadata;
use crate::error::WalletError;

use super::append_transaction_ledger;

pub(super) async fn serve(
    owner: Principal,
    metadata: Metadata,
    req: TransferRequest,
) -> Result<String, WalletError> {
    let txs = req.validate_address(metadata.network)?;

    // Log transfer info
    append_transaction_ledger(req.txs)?;

    send_p2pkh_transaction(
        metadata.ecdsa_key_id,
        metadata.network,
        principal_to_derivation_path(owner),
        &txs.txs,
    )
    .await
    .map(|txid| txid.to_string())
}

/// Send a transaction to bitcoin network that transfer the given amount and recipient
/// and the sender is the canister itself
pub async fn send_p2pkh_transaction(
    key_id: EcdsaKeyId,
    network: BitcoinNetwork,
    derivation_path: Vec<Vec<u8>>,
    txs: &[RecipientAmount],
) -> Result<Txid, WalletError> {
    // Get fee percentiles from ic api
    let fee_percentiles =
        bitcoin_get_current_fee_percentiles(GetCurrentFeePercentilesRequest { network })
            .await
            .unwrap()
            .0;

    let fee_per_byte = if fee_percentiles.is_empty() {
        // There are no fee percentiles if network is regtest. use default fee
        DEFAULT_FEE_MILLI_SATOSHI
    } else {
        // Choose the 50th percentile if len > 50
        if fee_percentiles.len() >= 50 {
            fee_percentiles[50]
        } else {
            *fee_percentiles.last().unwrap()
        }
    };

    // Fetch public key, p2pkh address, and utxos
    let sender_public_key =
        wallet::ecdsa::public_key(derivation_path.clone(), key_id.clone(), None).await?;

    // TODO: replace with stable store value
    let sender_address = wallet::bitcoins::public_key_to_p2pkh_address(network, &sender_public_key);
    ic_cdk::print(format!(
        "Sender address: {sender_address:?} ---------------------- \n"
    ));

    // Fetching UTXOs
    ic_cdk::print("Fetching UTXOs... \n");

    let sender_address = str_to_bitcoin_address(&sender_address, network)?;

    // TODO: UTXOs maybe very large, need to paginate
    let utxos = wallet::bitcoins::get_utxos(sender_address.to_string(), network, None)
        .await?
        .utxos;

    // let recipient = str_to_bitcoin_address(&recipient, network)?;

    // Build transaction
    let tx = utils::build_transaction(
        &sender_public_key,
        &sender_address,
        &utxos,
        txs,
        fee_per_byte,
    )
    .await?;

    // let tx_bytes = consensus::serialize(&tx);

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

    let signed_tx_bytes = consensus::serialize(&signed_tx);
    ic_cdk::print(format!("Signed tx: {:?} \n", hex::encode(&signed_tx_bytes)));

    let txid = signed_tx.compute_txid();

    ic_cdk::print(format!("Sending transaction... {txid:?}\n"));

    wallet::bitcoins::send_transaction(signed_tx_bytes, network).await?;

    ic_cdk::print("Transaction sent! \n");

    Ok(txid)
}
