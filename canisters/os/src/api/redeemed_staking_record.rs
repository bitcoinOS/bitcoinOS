

use ic_cdk::api::management_canister::main::CanisterId;

use crate::error::Error;

pub(super) async fn serve(staking_canister: CanisterId) -> Result<bool, Error> {
    let resp: Result<(bool, ), _> = ic_cdk::call(staking_canister, "redeemed_staking_record", ((), )).await;

    resp.map(|(b,)| b)
    .map_err(|e| Error::RedeemedStakingError(format!("{e:?}")))
}