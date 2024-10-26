use wallet::bitcoins::{build_unsigned_p2wpkh_transaction, sign_transaction_p2wpkh};
use wallet::domain::EcdsaKeyIdAndDerivationPath;
use wallet::tx::RecipientAmount;
use wallet::utils::{
    validate_recipient_amount_must_greater_than_1000, validate_recipient_cnt_must_less_than_100,
};

use wallet::utils::principal_to_derivation_path;

use bitcoin::{consensus, Transaction, Txid};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;

use crate::domain::Metadata;
use crate::error::WalletError;
use crate::repositories;

use super::{append_transaction_log, counter_increment_one};

pub(super) async fn serve(
    pk_bytes: &[u8],
    metadata: Metadata,
    txs: &[RecipientAmount],
) -> Result<String, WalletError> {
    validate_recipient_cnt_must_less_than_100(txs)?;
    validate_recipient_amount_must_greater_than_1000(txs)?;

    // Log transfer info
    append_transaction_log(txs).await?;

    // Transaction counter increment one
    counter_increment_one::serve();

    let sender = metadata.owner;
    let key_id = metadata.ecdsa_key_id.clone();
    let derivation_path = principal_to_derivation_path(sender);
    let p2wpkh_wallet = repositories::wallet::get_or_create_p2wpkh_wallet(metadata.clone()).await?;
    let sender_address = &p2wpkh_wallet.address;

    // Build transaction
    let (tx, input_amounts) = build_unsigned_p2wpkh_transaction(
        metadata.network,
        sender_address,
        pk_bytes,
        derivation_path.clone(),
        txs,
        bitcoin::EcdsaSighashType::All,
    )
    .await?;

    // Sign transaction
    // let signed_tx = sign_transaction_p2wpkh(
    //     pk_bytes,
    //     sender_address,
    //     tx,
    //     &input_amounts,
    //     bitcoin::EcdsaSighashType::All,
    //     metadata.ecdsa_key_id,
    //     principal_to_derivation_path(sender),
    // )
    // .await?;

    let signed_tx = sign_transaction_p2wpkh(
        pk_bytes,
        sender_address,
        tx,
        &input_amounts,
        bitcoin::EcdsaSighashType::All,
        EcdsaKeyIdAndDerivationPath {
            derivation_path,
            key_id,
        },
        wallet::ecdsa::sign_with_ecdsa_uncheck,
    )
    .await?;

    ic_cdk::print(format!(
        "signed tx is {:#?} ------------------\n",
        signed_tx
    ));

    // Send transaction
    send_transaction(&signed_tx, metadata.network)
        .await
        .map(|txid| txid.to_string())
}

async fn send_transaction(tx: &Transaction, network: BitcoinNetwork) -> Result<Txid, WalletError> {
    let signed_tx_bytes = consensus::serialize(tx);

    ic_cdk::print(format!(
        "Signed raw tx: {:#?} \n",
        hex::encode(&signed_tx_bytes)
    ));

    let txid = tx.compute_txid();

    ic_cdk::print(format!("Sending transaction... {txid:?}\n"));

    wallet::bitcoins::send_transaction(signed_tx_bytes, network).await?;

    ic_cdk::print("Transaction sent! \n");

    Ok(txid)
}
