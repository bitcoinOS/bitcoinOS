use candid::Principal;
use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, ecdsa::EcdsaKeyId};
use wallet::tx::{RawTransactionInfo, TransactionInfo};

use crate::error::StewardError;

pub async fn serve(
    raw_tx_info: RawTransactionInfo,
    key_id: EcdsaKeyId,
    wallet_canister: Principal,
    network: BitcoinNetwork,
) -> Result<String, StewardError> {
    let mut tx_info = TransactionInfo::try_from(raw_tx_info)?;

    // tx_info = wallet::utils::sign_transaction_multisig22(
    //     tx_info,
    //     &[wallet_canister.as_slice().to_vec()],
    //     key_id,
    //     wallet::domain::MultiSigIndex::Second,
    // )
    // .await?;

    // let txid = wallet::utils::send_transaction(&tx_info, network).await?;

    // Ok(txid.to_string())
    Ok("".to_string())
}
