use crate::repoistories::{
    wallet_owner_stable::WalletOwnerStableRepository, WalletOwnerRepository,
};

pub fn execute(repo: &WalletOwnerStableRepository) -> u64 {
    repo.count_wallet()
}
