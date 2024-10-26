use crate::{domain::Metadata, error::WalletError};

pub async fn serve(metadata: Metadata) -> Result<String, WalletError> {
    crate::repositories::wallet::get_or_create_p2wpkh_wallet(metadata)
        .await
        .map(|w| w.address.to_string())
}
