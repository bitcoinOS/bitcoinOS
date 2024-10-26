use crate::repositories;
use wallet::{
    domain::{request::UpdateStakingPoolStatusRequest, staking::StakingPoolInfo},
    error::StakingError,
};

pub(crate) async fn serve(req: UpdateStakingPoolStatusRequest) -> Result<String, StakingError> {
    let updated_resp = update_staking_pool_status(req.clone()).await?;

    let staking_pool_canister = req.staking_pool_canister;
    let staking_pool_info = repositories::staking_pool::get(&staking_pool_canister).ok_or(
        StakingError::StakingPoolNotFound(staking_pool_canister.to_string()),
    )?;

    let new_info = StakingPoolInfo {
        status: req.status.into(),
        ..staking_pool_info
    };

    repositories::staking_pool::save(new_info);

    Ok(updated_resp)
}

async fn update_staking_pool_status(
    req: UpdateStakingPoolStatusRequest,
) -> Result<String, StakingError> {
    let resp: Result<(String,), _> = ic_cdk::call(
        req.staking_pool_canister,
        "update_staking_pool_status",
        (req,),
    )
    .await;

    resp.map(|(r,)| r)
        .map_err(|e| StakingError::OsCallError(e.1))
}
