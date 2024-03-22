use crate::{
    domain::Action,
    error::Error,
    repoistories::{wallet_action_stable::WalletActionStableRepository, WalletActionRepository},
};

pub fn execute(
    repo: WalletActionStableRepository,
    operator: candid::Principal,
    action: Action,
    op_time: u64,
) -> Result<u64, Error> {
    repo.append_wallet_action(operator, action, op_time)
}
