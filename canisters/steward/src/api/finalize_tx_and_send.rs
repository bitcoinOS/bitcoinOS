use base::tx::RawTransactionInfo;
use candid::Principal;
use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, ecdsa::EcdsaKeyId};

use crate::error::StewardError;

pub async fn serve(
    raw_tx_info: RawTransactionInfo,
    key_id: EcdsaKeyId,
    wallet_canister: Principal,
    network: BitcoinNetwork,
) -> Result<String, StewardError> {
    let mut tx_info = base::tx::TransactionInfo::try_from(raw_tx_info)?;

    tx_info = base::utils::sign_transaction(
        tx_info,
        &[wallet_canister.as_slice().to_vec()],
        key_id,
        base::domain::MultiSigIndex::Second,
    )
    .await?;

    base::utils::send_transaction(&tx_info, network).await?;

    Ok(tx_info.tx.txid().to_string())
}
