use candid::Principal;

use crate::{domain::WalletOwner, error::Error, services, WALLET_OWNER};

pub fn serve(
    owner: Principal,
    canister_id: Principal,
    created_at: u64,
) -> Result<Option<WalletOwner>, Error> {
    WALLET_OWNER.with(|w| {
        services::insert_wallet_owner::execute(&mut w.into(), owner, canister_id, created_at)
    })
}
