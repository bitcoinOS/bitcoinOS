use crate::{domain::Metadata, repositories::wallet as cwallet};
use wallet::error::StakingError;
/// Returns the P2PKH address of this canister
/// if P2PKH address not exist, create a new one and and save it to stable storage
pub(super) async fn serve(metadata: Metadata) -> Result<String, StakingError> {
    cwallet::get_or_create_p2pkh_wallet(metadata)
        .await
        .map(|w| w.address.to_string())
}
