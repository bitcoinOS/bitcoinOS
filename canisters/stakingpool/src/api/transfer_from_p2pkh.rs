use bitcoin::consensus;

use bitcoin::Transaction;
use bitcoin::Txid;

use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use wallet::bitcoins;
use wallet::tx::RecipientAmount;
use wallet::utils::validate_recipient_amount_must_greater_than_1000;
use wallet::utils::validate_recipient_cnt_must_less_than_100;
use wallet::utils::{self, principal_to_derivation_path};
use wallet::{constants::DEFAULT_FEE_MILLI_SATOSHI, utils::str_to_bitcoin_address};

use crate::domain::Metadata;
use wallet::error::StakingError;

pub(super) async fn serve(
    public_key: &[u8],
    metadata: Metadata,
    txs: &[RecipientAmount],
) -> Result<String, StakingError> {
    validate_recipient_cnt_must_less_than_100(txs)?;
    validate_recipient_amount_must_greater_than_1000(txs)?;

    // let txs = req.validate_address(metadata.network)?;

    // Log transfer info
    // append_transaction_log(&req.txs).await?;

    // // Transaction counter increment one
    // counter_increment_one::serve();

    send_p2pkh_transaction(public_key, metadata, txs)
        .await
        .map(|txid| txid.to_string())
}

/// Send a transaction to bitcoin network that transfer the given amount and recipient
/// and the sender is the canister itself
pub async fn send_p2pkh_transaction(
    public_key: &[u8],
    metadata: Metadata,
    txs: &[RecipientAmount],
) -> Result<Txid, StakingError> {
    let network = metadata.network;
    let key_id = metadata.ecdsa_key_id.clone();
    let derivation_path = principal_to_derivation_path(metadata.owner);

    // Get fee per byte
    let fee_per_byte = bitcoins::get_fee_per_byte(network, DEFAULT_FEE_MILLI_SATOSHI).await?;

    // Fetch public key, p2pkh address
    let sender_address = bitcoins::public_key_to_p2pkh_address(network, public_key);
    let sender_address = str_to_bitcoin_address(&sender_address, network)?;

    ic_cdk::print(format!(
        "Sender address: {sender_address:?} ---------------------- \n"
    ));

    // Fetching UTXOs
    ic_cdk::print("Fetching UTXOs... \n");

    // FIXME: UTXOs maybe very large, need to paginate
    let utxos = wallet::bitcoins::get_utxos(sender_address.to_string(), network, None)
        .await?
        .utxos;

    // Build transaction
    let tx =
        utils::build_transaction(public_key, &sender_address, &utxos, txs, fee_per_byte).await?;

    // Sign the transaction
    let signed_tx = utils::sign_transaction_p2pkh(
        public_key,
        &sender_address,
        tx,
        key_id,
        derivation_path,
        wallet::ecdsa::sign_with_ecdsa_uncheck,
    )
    .await?;

    send_transaction(&signed_tx, network).await
}

async fn send_transaction(tx: &Transaction, network: BitcoinNetwork) -> Result<Txid, StakingError> {
    let signed_tx_bytes = consensus::serialize(tx);
    ic_cdk::print(format!("Signed tx: {:?} \n", hex::encode(&signed_tx_bytes)));

    let txid = tx.compute_txid();

    ic_cdk::print(format!("Sending transaction... {txid:?}\n"));

    wallet::bitcoins::send_transaction(signed_tx_bytes, network).await?;

    ic_cdk::print("Transaction sent! \n");

    Ok(txid)
}
