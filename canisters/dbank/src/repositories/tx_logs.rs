use wallet::{
    tx::RecipientAmount,
    utils::{ic_caller, ic_time},
};

use crate::{context::STATE, domain::TransactionLog, error::DBankError};

pub(crate) fn list() -> Vec<TransactionLog> {
    STATE.with_borrow(|s| s.tx_logs.iter().collect())
}

pub(crate) fn get(idx: u64) -> Option<TransactionLog> {
    STATE.with_borrow(|s| s.tx_logs.get(idx))
}

pub(crate) fn append_transaction_log(log: &TransactionLog) -> Result<(), DBankError> {
    STATE.with_borrow_mut(|s| {
        s.tx_logs
            .append(log)
            .map_err(|e| DBankError::AppendTransferLogError(format!("{:?}", e)))?;

        Ok(())
    })
}

pub(crate) fn build_and_append_transaction_log(txs: &[RecipientAmount]) -> Result<(), DBankError> {
    let sender = ic_caller();
    let send_time = ic_time();
    let log = &TransactionLog {
        txs: txs.iter().map(|ra| ra.into()).collect(),
        sender,
        send_time,
    };

    append_transaction_log(log)
}
