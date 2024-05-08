use crate::{domain::WalletInfo, error::Error, repositories};

pub fn serve(wallet_info: WalletInfo) -> Result<(), Error> {
    repositories::wallet_owner::save(
        wallet_info.owner,
        wallet_info.wallet_canister,
        wallet_info.created_at,
    )?;

    repositories::wallet_info::save(wallet_info)
}
