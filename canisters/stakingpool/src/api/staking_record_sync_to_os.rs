use ic_cdk::api::management_canister::main::CanisterId;
use wallet::domain::staking::StakingRecord;

use wallet::error::StakingError;

pub(super) async fn serve(
    os_canister: CanisterId,
    record: StakingRecord,
) -> Result<bool, StakingError> {
    let resp: Result<(bool,), _> =
        ic_cdk::call(os_canister, "register_staking_record", (record,)).await;

    resp.map(|(r,)| r)
        .map_err(|e| StakingError::OsCallError(format!("{e:?}")))
}
