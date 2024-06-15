use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_cdk::api::management_canister::main::CanisterId;
use wallet::domain::request::{FinalizeRequest, TransferRequest};
use wallet::domain::response::FinalizeTransactionResponse;
use wallet::domain::MultiSigIndex;
use wallet::tx::RawTransactionInfo;
use wallet::tx::RecipientAmount;
use wallet::utils::{
    principal_to_derivation_path, validate_recipient_amount_must_greater_than_1000,
    validate_recipient_cnt_must_less_than_100,
};

use crate::domain::Metadata;
use crate::error::StakingError;
use crate::repositories;

pub(super) async fn serve(
    metadata: Metadata,
    req: TransferRequest,
) -> Result<String, StakingError> {
    validate_recipient_cnt_must_less_than_100(&req.txs)?;
    validate_recipient_amount_must_greater_than_1000(&req.txs)?;

    let network = metadata.network;
    let steward_canister = metadata.steward_canister;

    let txs = req.validate_address(network)?;

    let tx_info_bytes = init_transfer_request(metadata, &txs.txs).await?;

    ic_cdk::print("After init_send_request --------------\n");
    ic_cdk::print(format!("tx_info is: {:?} ---------------\n", tx_info_bytes));

    // Send transaction
    finalize_and_send(steward_canister, network, tx_info_bytes).await
}

async fn init_transfer_request(
    metadata: Metadata,
    txs: &[RecipientAmount],
) -> Result<RawTransactionInfo, StakingError> {
    let sender = metadata.owner;
    let key_id = metadata.ecdsa_key_id.clone();
    let network = metadata.network;
    let wallet = repositories::wallet::get_or_create_p2wsh_multisig22_wallet(metadata).await?;

    let sighash_type = bitcoin::EcdsaSighashType::All;
    // Build an unsigned transaction
    let tx_info = wallet::bitcoins::build_unsigned_transaction_p2wsh_multisig22(
        &wallet,
        network,
        txs,
        sighash_type,
    )
    .await?;

    // Sign the transaction
    let derivation_path = &principal_to_derivation_path(sender);

    // let derivation_path2 = principal_to_derivation_path(ic_cdk::id());

    let half_signed_tx = wallet::bitcoins::sign_transaction_p2wsh_multisig22(
        &tx_info,
        key_id,
        derivation_path,
        MultiSigIndex::First,
        sighash_type,
    )
    .await?;

    Ok(half_signed_tx.into())
}

/// Send transaction to Steward canister to finalize and commit transaction to bitcoin network
async fn finalize_and_send(
    steward_canister: CanisterId,
    network: BitcoinNetwork,
    tx_info_bytes: RawTransactionInfo,
) -> Result<String, StakingError> {
    ic_cdk::print(format!(
        "The steward canister is {:?}",
        steward_canister.to_string()
    ));

    let resp: Result<(FinalizeTransactionResponse,), _> = ic_cdk::call(
        steward_canister,
        "finalize_tx_and_send",
        (FinalizeRequest {
            network,
            tx_info_bytes,
        },),
    )
    .await;

    ic_cdk::print(format!(
        "result from steward is: {:?} ----------------\n ",
        resp
    ));

    match resp {
        Ok((resp,)) => {
            if resp.txid.is_some() {
                Ok(resp.txid.unwrap())
            } else {
                Err(StakingError::StewardCallError(resp.error_msg.unwrap_or(
                    "Finalize and send transaction error!".to_string(),
                )))
            }
        }
        Err((code, msg)) => Err(StakingError::StewardCallError(format!("{code:?}: {msg:?}"))),
    }
}
