use crate::{context::STATE, domain::WalletOperationEvent};

pub(crate) fn list() -> Vec<WalletOperationEvent> {
    STATE.with_borrow(|s| s.wallet_logs.iter().collect())
}

pub(crate) fn get(idx: u64) -> Option<WalletOperationEvent> {
    STATE.with_borrow(|s| s.wallet_logs.get(idx))
}

// pub(crate) fn append_wallet_log(log: &WalletInfo) -> Result<(), DBankError> {
//     STATE.with(|s| {
//         s.borrow_mut()
//             .wallet_logs
//             .append(&WalletOperationEvent::CreateWallet(CreateWalletEvent {
//                 wallet_info: log.clone(),
//             }))
//             .map_err(|e| DBankError::AppendTransferLogError(format!("{:?}", e)))?;

//         Ok(())
//     })
// }
