use ic_cdk::api::management_canister::main::CanisterId;
use wallet::domain::staking::{InitStakingPoolArgument, StakingPoolInfo};

use crate::{context::STATE, error::Error};

pub(crate) fn get(staking_canister: &CanisterId) -> Option<StakingPoolInfo> {
    STATE.with_borrow(|s| s.staking_pools.get(staking_canister))
}

pub(crate) fn list_staking_pool() -> Vec<StakingPoolInfo> {
    STATE.with(|s| {
        s.borrow()
            .staking_pools
            .iter()
            .map(|(_, v)| v.clone())
            .collect()
    })
}

pub(crate) fn save(pool_info: StakingPoolInfo) {
    STATE.with_borrow_mut(|s| {
        s.staking_pools
            .insert(pool_info.staking_pool_canister, pool_info)
    });
}

pub(crate) fn create_staking_pool(
    staking_pool_canister: CanisterId,
    created_at: u64,
    bitcoin_address: String,
    arg: InitStakingPoolArgument,
) -> Result<StakingPoolInfo, Error> {
    STATE.with(|s| {
        let state = &mut s.borrow_mut();
        let staking_pools = &mut state.staking_pools;

        if staking_pools.contains_key(&staking_pool_canister) {
            Err(Error::StakingPoolAlreadyExists {
                staking_pool_id: staking_pool_canister.to_string(),
            })
        } else {
            let staking_pool = StakingPoolInfo {
                name: arg.name,
                staking_pool_canister,
                description: arg.description,
                network: arg.network,
                annual_interest_rate: arg.annual_interest_rate,
                duration_in_day: arg.duration_in_day,
                os_canister: arg.os_canister,
                created_at,
                bitcoin_address,
                steward_canister: arg.steward_canister,
                status: arg.status.into(),
                start_time: arg.start_time,
                // stake_end_time: arg.stake_end_time,
                end_time: arg.end_time,
                fund_management: arg.fund_management.into(),
                boost_rate: arg.boost_rate,
                minimum_stake_amount: arg.minimum_stake_amount,
            };

            staking_pools.insert(staking_pool_canister, staking_pool.clone());
            Ok(staking_pool)
        }
    })
}

pub(crate) fn update_bitcoin_address(
    canister_id: CanisterId,
    bitcoin_address: String,
) -> Result<StakingPoolInfo, Error> {
    STATE.with_borrow_mut(|state| {
        let staking_pools = &mut state.staking_pools;

        match staking_pools.get(&canister_id) {
            Some(info) => {
                let new_info = StakingPoolInfo {
                    bitcoin_address,
                    ..info
                };
                staking_pools.insert(canister_id, new_info.clone());

                Ok(new_info)
            }
            None => Err(Error::UnAuthorized(canister_id.to_string())),
        }
    })
}
