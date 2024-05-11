use ic_cdk::api::management_canister::main::CanisterId;

use crate::context::STATE;

pub fn save(wallet_canister: CanisterId, staked_time: u64) -> Option<u64> {
    STATE.with_borrow_mut(|s| s.stakers.insert(wallet_canister, staked_time))
}
