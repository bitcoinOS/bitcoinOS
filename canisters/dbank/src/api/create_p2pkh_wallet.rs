use candid::Principal;

use crate::{domain::Metadata, error::DBankError, repositories};

/// Returns the P2PKH address of this canister
/// if P2PKH address not exist, create a new one and and save it to stable storage
pub(super) async fn serve(
    seq_in_os: u64,
    metadata: Metadata,
    wallet_owner: Principal,
    name: String,
) -> Result<String, DBankError> {
    repositories::wallet::get_or_create_p2pkh_wallet(seq_in_os, metadata, wallet_owner, name)
        .await
        .map(|w| w.address.to_string())
}
