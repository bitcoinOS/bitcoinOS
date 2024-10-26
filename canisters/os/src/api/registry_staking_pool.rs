use ic_cdk::api::management_canister::main::CanisterId;
use wallet::domain::staking::{InitStakingPoolArgument, StakingPoolInfo};

use crate::{error::Error, repositories};

pub(crate) fn serve(
    staking_pool_canister: CanisterId,
    created_at: u64,
    bitcoin_address: String,
    arg: InitStakingPoolArgument,
) -> Result<StakingPoolInfo, Error> {
    repositories::staking_pool::create_staking_pool(
        staking_pool_canister,
        created_at,
        bitcoin_address,
        arg,
    )
}
