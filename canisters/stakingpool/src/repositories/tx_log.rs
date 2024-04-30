use wallet::utils::{ic_caller, ic_time};

use crate::{
    context::STATE,
    domain::{request::TransferInfo, TransactionLog},
    error::WalletError,
};

pub(crate) fn append_transaction_log(log: &TransactionLog) -> Result<(), WalletError> {
    STATE.with(|s| {
        s.borrow_mut()
            .logs
            .append(log)
            .map_err(|e| WalletError::AppendTransferLogError(format!("{:?}", e)))?;

        Ok(())
    })
}

pub(crate) fn build_and_append_transaction_log(txs: Vec<TransferInfo>) -> Result<(), WalletError> {
    let sender = ic_caller();
    let send_time = ic_time();
    let log = TransactionLog {
        txs,
        sender,
        send_time,
    };

    append_transaction_log(&log)
}
