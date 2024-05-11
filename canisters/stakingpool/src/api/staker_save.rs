use ic_cdk::api::management_canister::main::CanisterId;

use crate::repositories;

pub(super) fn serve(wallet_canister: CanisterId, staked_time: u64) -> Option<u64> {
    repositories::staker::save(wallet_canister, staked_time)
}
