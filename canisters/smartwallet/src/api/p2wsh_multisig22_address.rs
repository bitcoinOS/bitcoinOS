use crate::{domain::Metadata, error::WalletError, repositories::wallet};

/// Returns the P2PKH address of this canister
/// if P2PKH address not exist, create a new one and and save it to stable storage
pub(super) async fn serve(metadata: Metadata) -> Result<String, WalletError> {
    wallet::get_or_create_p2wsh_multisig22_wallet(metadata)
        .await
        .map(|w| w.address.to_string())
}
