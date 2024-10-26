use candid::Principal;

use crate::{domain::Metadata, error::DBankError, repositories};

pub async fn serve(
    seq_in_os: u64,
    metadata: Metadata,
    wallet_owner: Principal,
    name: String,
) -> Result<String, DBankError> {
    repositories::metadata::update_current_seq_in_os(seq_in_os)?;

    repositories::wallet::get_or_create_p2wpkh_wallet(seq_in_os, metadata, wallet_owner, name)
        .await
        .map(|w| w.address.to_string())
}
