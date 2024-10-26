use crate::{domain::Metadata, repositories::wallet as swallet};
use wallet::error::StakingError;

/// Returns the P2WSH multisig 2x2 address of this canister and steward canister
/// if P2WSH address not exist, create a new one and and save it to stable storage
pub(super) async fn serve(metadata: Metadata) -> Result<String, StakingError> {
    swallet::get_or_create_p2wsh_multisig22_wallet(metadata)
        .await
        .map(|w| w.address.to_string())
}
