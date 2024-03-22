use crate::{
    domain::WalletOwner,
    error::Error,
    repoistories::{wallet_owner_stable::WalletOwnerStableRepository, WalletOwnerRepository},
};

pub fn execute(
    repo: &mut WalletOwnerStableRepository,
    owner: candid::Principal,
    canister_id: candid::Principal,
    created_at: u64,
) -> Result<Option<WalletOwner>, Error> {
    repo.insert_wallet_owner(owner, canister_id, created_at)
}
