use crate::{domain::WalletInfo, error::Error, repositories};

pub fn serve(wallet_info: WalletInfo) -> Result<(), Error> {
    repositories::wallet_info::save(wallet_info)
}
