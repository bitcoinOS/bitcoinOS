use ic_cdk::api::management_canister::main::CanisterId;
use wallet::domain::staking::StakingPoolInfo;

use crate::{error::Error, repositories};

pub(super) fn serve(
    staking_pool_canister: CanisterId,
    bitcoin_address: String,
) -> Result<StakingPoolInfo, Error> {
    repositories::staking_pool::update_bitcoin_address(staking_pool_canister, bitcoin_address)
}
