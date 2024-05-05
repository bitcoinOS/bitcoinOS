use ic_cdk::api::management_canister::main::CanisterId;

use crate::{
    domain::{request::RegisterStakingRequest, StakingRecord},
    error::WalletError,
};

pub(super) async fn serve(req: RegisterStakingRequest) -> Result<StakingRecord, WalletError> {
    let resp: Result<(StakingRecord,), _> =
        ic_cdk::call(req.staking_canister, "register_staking_record", (req,)).await;

    resp.map(|(r,)| r)
        .map_err(|e| WalletError::RegisterStakingRecordError(format!("{e:?}")))
}
