use crate::{domain::Metadata, error::StakingError, repositories::wallet};

/// Returns the P2WSH multisig 2x2 address of this canister and steward canister
/// if P2WSH address not exist, create a new one and and save it to stable storage
pub(super) async fn serve(metadata: Metadata) -> Result<String, StakingError> {
    wallet::get_or_create_p2wsh_multisig22_wallet(metadata)
        .await
        .map(|w| w.address.to_string())
}
