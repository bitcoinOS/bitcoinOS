use wallet::{
    tx::RecipientAmount,
    utils::{ic_caller, ic_time},
};

use crate::{context::STATE, domain::TransactionLog, error::WalletError};

pub(crate) fn append_transaction_log(log: &TransactionLog) -> Result<(), WalletError> {
    STATE.with(|s| {
        s.borrow_mut()
            .logs
            .append(log)
            .map_err(|e| WalletError::AppendTransferLogError(format!("{:?}", e)))?;

        Ok(())
    })
}

pub(crate) fn build_and_append_transaction_log(txs: &[RecipientAmount]) -> Result<(), WalletError> {
    let sender = ic_caller();
    let send_time = ic_time();
    let log = &TransactionLog {
        txs: txs.iter().map(|ra| ra.into()).collect(),
        sender,
        send_time,
    };

    append_transaction_log(log)
}
