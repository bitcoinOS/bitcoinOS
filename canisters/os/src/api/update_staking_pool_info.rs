use crate::repositories;
use wallet::{
    domain::{
        request::UpdateStakingPoolInfoRequest, response::UpdateStakingPoolInfoResponse,
        staking::StakingPoolInfo,
    },
    error::StakingError,
};

pub(crate) async fn serve(
    req: UpdateStakingPoolInfoRequest,
) -> Result<UpdateStakingPoolInfoResponse, StakingError> {
    let updated_resp = update_staking_pool_info(req.clone()).await?;

    let staking_pool_canister = req.staking_pool_canister;
    let staking_pool_info = repositories::staking_pool::get(&staking_pool_canister).ok_or(
        StakingError::StakingPoolNotFound(staking_pool_canister.to_string()),
    )?;

    let new_info = StakingPoolInfo {
        name: req.name,
        description: req.description,
        annual_interest_rate: req.annual_interest_rate,
        duration_in_day: req.duration_in_day,
        status: req.status.into(),
        start_time: req.start_time,
        end_time: req.end_time,
        ..staking_pool_info
    };

    repositories::staking_pool::save(new_info);

    Ok(updated_resp)
}

async fn update_staking_pool_info(
    req: UpdateStakingPoolInfoRequest,
) -> Result<UpdateStakingPoolInfoResponse, StakingError> {
    let resp: Result<(UpdateStakingPoolInfoResponse,), _> = ic_cdk::call(
        req.staking_pool_canister,
        "update_staking_pool_info",
        (req,),
    )
    .await;

    resp.map(|(r,)| r)
        .map_err(|e| StakingError::OsCallError(e.1))
}
