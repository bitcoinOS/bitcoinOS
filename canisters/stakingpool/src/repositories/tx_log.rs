use wallet::utils::{ic_caller, ic_time};

use crate::{
    context::STATE,
    domain::{request::RedeemRequest, RedeemLog},
    error::StakingError,
};

pub(crate) fn build_and_append_redeem_log(req: RedeemRequest) -> Result<(), StakingError> {
    let sender = ic_caller();
    let send_time = ic_time();
    let log = RedeemLog {
        req,
        sender,
        send_time,
    };

    append_redeem_log(&log)
}

fn append_redeem_log(log: &RedeemLog) -> Result<(), StakingError> {
    STATE.with(|s| {
        s.borrow_mut()
            .redeem_logs
            .append(log)
            .map_err(|e| StakingError::AppendRedeemLogError(format!("{:?}", e)))?;

        Ok(())
    })
}
