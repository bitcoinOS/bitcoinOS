use crate::{
    domain::WalletOwner,
    repoistories::{wallet_owner_stable::WalletOwnerStableRepository, WalletOwnerRepository},
};

pub fn execute(repo: &WalletOwnerStableRepository) -> Vec<WalletOwner> {
    repo.list_wallet()
}
