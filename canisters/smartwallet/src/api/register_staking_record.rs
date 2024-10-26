use wallet::domain::{request::RegisterStakingRecordRequest, staking::StakingRecord};

use crate::error::WalletError;

pub(super) async fn serve(req: RegisterStakingRecordRequest) -> Result<StakingRecord, WalletError> {
    let resp: Result<(StakingRecord,), _> =
        ic_cdk::call(req.staking_canister, "register_staking_record", (req,)).await;

    resp.map(|(r,)| r)
        .map_err(|e| WalletError::RegisterStakingRecordError(format!("{e:?}")))
}
