use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, main::CanisterId};

use crate::{context::STATE, domain::StakingPoolInfo, error::Error};

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

#[allow(clippy::too_many_arguments)]
pub(crate) fn create_staking_pool(
    canister_id: CanisterId,
    network: BitcoinNetwork,
    os_canister: CanisterId,
    created_at: u64,
    name: String,
    description: String,
    annual_interest_rate: u16,
    duration_in_millisecond: u64,
    bitcoin_address: String,
    steward_canister: CanisterId,
) -> Result<StakingPoolInfo, Error> {
    STATE.with(|s| {
        let state = &mut s.borrow_mut();
        let staking_pools = &mut state.staking_pools;

        if staking_pools.contains_key(&canister_id) {
            Err(Error::StakingPoolAlreadyExists {
                staking_pool_id: canister_id.to_string(),
            })
        } else {
            let staking_pool = StakingPoolInfo {
                name,
                staking_pool_canister: canister_id,
                description,
                network,
                annual_interest_rate,
                duration_in_day: duration_in_millisecond,
                os_canister,
                created_at,
                bitcoin_address,
                steward_canister,
            };

            staking_pools.insert(canister_id, staking_pool.clone());
            Ok(staking_pool)
        }
    })
}

#[allow(clippy::too_many_arguments)]
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
