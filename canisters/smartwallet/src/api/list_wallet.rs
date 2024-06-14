use crate::{domain::response::ListWalletResponse, repositories};

pub(super) fn serve() -> Vec<ListWalletResponse> {
    repositories::wallet::list_wallet()
        .iter()
        .map(|(key, wallet)| ListWalletResponse {
            key: key.to_owned(),
            wallet: wallet.to_owned(),
        })
        .collect()
}
