use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, main::CanisterId};

use crate::{domain::StakingPoolInfo, error::Error, repositories};

pub(crate) fn serve(
    staking_pool_id: CanisterId,
    network: BitcoinNetwork,
    os_canister: CanisterId,
    created_at: u64,
    name: String,
    description: String,
    annual_interest_rate: u64,
    duration_in_millisecond: u64,
    bitcoin_address: String,
) -> Result<StakingPoolInfo, Error> {
    repositories::staking_pool::create_staking_pool(
        staking_pool_id,
        network,
        os_canister,
        created_at,
        name,
        description,
        annual_interest_rate,
        duration_in_millisecond,
        bitcoin_address,
    )
}
