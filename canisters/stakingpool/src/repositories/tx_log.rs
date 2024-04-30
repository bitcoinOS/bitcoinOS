use wallet::utils::{ic_caller, ic_time};

use crate::{
    context::STATE,
    domain::{request::RedeemRequest, RedeemLog},
    error::WalletError,
};

pub(crate) fn append_redeem_log(log: &RedeemLog) -> Result<(), WalletError> {
    STATE.with(|s| {
        s.borrow_mut()
            .redeem_logs
            .append(log)
            .map_err(|e| WalletError::AppendTransferLogError(format!("{:?}", e)))?;

        Ok(())
    })
}

pub(crate) fn build_and_append_redeem_log(req: RedeemRequest) -> Result<(), WalletError> {
    let sender = ic_caller();
    let send_time = ic_time();
    let log = RedeemLog {
        req,
        sender,
        send_time,
    };

    append_redeem_log(&log)
}
