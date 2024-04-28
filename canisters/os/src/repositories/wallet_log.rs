use crate::{context::STATE, domain::WalletAction, error::Error};

pub(crate) fn append_wallet_action(
    operator: candid::Principal,
    action: crate::domain::Action,
    op_time: u64,
) -> Result<u64, Error> {
    let wallet_action = WalletAction {
        operator,
        action,
        op_time,
    };

    STATE.with(|s| {
        s.borrow_mut()
            .logs
            .append(&wallet_action)
            .map_err(|e| Error::WriteError {
                msg: format!("{e:?}"),
            })
    })
}

pub(crate) fn get_wallet_action(idx: u64) -> Option<WalletAction> {
    STATE.with(|s| s.borrow().logs.get(idx))
}
