use bitcoin::{consensus, EcdsaSighashType, Transaction, Txid};
use candid::Principal;
use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, ecdsa::EcdsaKeyId};
use wallet::{tx::{RawTransactionInfo, TransactionInfo}, utils::principal_to_derivation_path};

use crate::error::StewardError;

pub async fn serve(
    raw_tx_info: RawTransactionInfo,
    key_id: EcdsaKeyId,
    wallet_canister: Principal,
    network: BitcoinNetwork,
) -> Result<String, StewardError> {
    let mut tx_info = TransactionInfo::try_from(raw_tx_info)?;

    tx_info = wallet::bitcoins::sign_transaction_p2wsh_multisig22(
        &tx_info,
        key_id,
        &principal_to_derivation_path(wallet_canister),
        wallet::domain::MultiSigIndex::Second,
        EcdsaSighashType::All,
    )
    .await?;

    let txid = send_transaction(&tx_info.tx, network).await;

    txid.map(|t| t.to_string())
}

async fn send_transaction(tx: &Transaction, network: BitcoinNetwork) -> Result<Txid, StewardError> {
    let signed_tx_bytes = consensus::serialize(tx);
    ic_cdk::print(format!("Signed tx: {:?} \n", hex::encode(&signed_tx_bytes)));

    let txid = tx.compute_txid();

    ic_cdk::print(format!("Sending transaction... {txid:?}\n"));

    wallet::bitcoins::send_transaction(signed_tx_bytes, network).await?;

    ic_cdk::print("Transaction sent! \n");

    Ok(txid)
}