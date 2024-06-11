use wallet::utils;

use wallet::utils::principal_to_derivation_path;

use bitcoin::{consensus, Transaction, Txid};

use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;

use crate::domain::Metadata;
use crate::repositories;
use crate::{domain::request::TransferRequest, error::WalletError};

use super::{
    append_transaction_log, counter_increment_one,
    validate_recipient_amount_must_greater_than_1000, validate_recipient_cnt_must_less_than_100,
};

pub(super) async fn serve(
    pk_bytes: &[u8],
    metadata: Metadata,
    req: TransferRequest,
) -> Result<String, WalletError> {
    validate_recipient_cnt_must_less_than_100(&req.txs)?;
    validate_recipient_amount_must_greater_than_1000(&req.txs)?;

    let txs = req.validate_address(metadata.network)?;

    // Log transfer info
    append_transaction_log(&req.txs).await?;

    // Transaction counter increment one
    counter_increment_one::serve();

    let sender = metadata.owner;
    let p2wpkh_wallet = repositories::wallet::get_or_create_p2wpkh_wallet(metadata.clone()).await?;
    let sender_address = &p2wpkh_wallet.address;

    // Build transaction
    let (tx_info, input_amounts) = wallet::utils::build_unsigned_p2wpkh_transaction(
        metadata.network,
        &p2wpkh_wallet,
        &txs.txs,
        bitcoin::EcdsaSighashType::All,
    )
    .await?;

    // Sign transaction
    let signed_tx = utils::sign_transaction_p2wpkh(
        pk_bytes,
        sender_address,
        tx_info,
        &input_amounts,
        bitcoin::EcdsaSighashType::All,
        metadata.ecdsa_key_id,
        principal_to_derivation_path(sender),
    )
    .await?;

    // Send transaction
    send_transaction(&signed_tx.tx, metadata.network)
        .await
        .map(|txid| txid.to_string())
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
