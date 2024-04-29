use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, main::CanisterId};

use crate::{
    context::STATE,
    domain::{request::InitStakingPoolArgument, StakingPoolInfo},
    error::Error,
};

pub(crate) fn list_staking_pool() -> Vec<StakingPoolInfo> {
    STATE.with(|s| {
        s.borrow()
            .staking_pools
            .iter()
            .map(|(_, v)| v.clone())
            .collect()
    })
}

pub(crate) fn create_staking_pool(
    canister_id: CanisterId,
    network: BitcoinNetwork,
    os_canister: CanisterId,
    created_at: u64,
    arg: InitStakingPoolArgument,
) -> Result<Option<StakingPoolInfo>, Error> {
    STATE.with(|s| {
        let state = &mut s.borrow_mut();
        let staking_pools = &mut state.staking_pools;

        if staking_pools.contains_key(&canister_id) {
            Err(Error::StakingPoolAlreadyExists {
                staking_pool_id: canister_id.to_string(),
            })
        } else {
            let staking_pool = StakingPoolInfo {
                name: arg.name,
                staking_pool_canister: canister_id,
                description: arg.description,
                network,
                annual_interest_rate: arg.annual_interest_rate,
                os_canister,
                created_at,
            };

            staking_pools.insert(canister_id, staking_pool.clone());
            Ok(Some(staking_pool))
        }
    })
}
