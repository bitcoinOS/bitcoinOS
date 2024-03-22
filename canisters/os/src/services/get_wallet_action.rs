use crate::{
    domain::WalletAction,
    repoistories::{wallet_action_stable::WalletActionStableRepository, WalletActionRepository},
};

pub fn execute(repo: WalletActionStableRepository, idx: u64) -> Option<WalletAction> {
    repo.get(idx)
}
