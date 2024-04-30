use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, main::CanisterId};

use crate::{
    domain::{request::InitStakingPoolArgument, StakingPoolInfo},
    error::Error,
    repositories,
};

pub(crate) fn serve(
    staking_pool_id: CanisterId,
    network: BitcoinNetwork,
    os_canister: CanisterId,
    created_at: u64,
    arg: InitStakingPoolArgument,
    bitcoin_address: String,
) -> Result<StakingPoolInfo, Error> {
    repositories::staking_pool::create_staking_pool(
        staking_pool_id,
        network,
        os_canister,
        created_at,
        arg,
        bitcoin_address,
    )
}
