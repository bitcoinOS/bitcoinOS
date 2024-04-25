use base::utils::principal_to_derivation_path;
use candid::Principal;

use crate::{domain::Metadata, error::WalletError};

pub(super) async fn serve(owner: Principal, metadata: Metadata) -> Result<String, WalletError> {
    base::bitcoins::get_p2pkh_address(
        metadata.network,
        principal_to_derivation_path(owner),
        metadata.ecdsa_key_id,
        None,
    )
    .await
    .map_err(|e| e.into())
}
