use candid::Principal;

use crate::{domain::WalletInfo, error::Error, repositories};

pub fn serve(
    owner: Principal,
    canister_id: Principal,
    wallet_info: WalletInfo,
) -> Result<(), Error> {
    // repositories::wallet_owner::create_wallet_owner(owner, canister_id, wallet_info.created_at)?;

    repositories::wallet_info::save(owner, canister_id, wallet_info)
}
