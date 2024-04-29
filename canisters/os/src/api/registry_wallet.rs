use candid::Principal;

use crate::{domain::WalletOwner, error::Error, repositories};

pub fn serve(
    owner: Principal,
    canister_id: Principal,
    created_at: u64,
) -> Result<Option<WalletOwner>, Error> {
    repositories::wallet_owner::create_wallet_owner(owner, canister_id, created_at)
}
