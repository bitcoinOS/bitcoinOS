use ic_cdk::api::management_canister::main::CanisterId;

use crate::repositories;

pub(super) fn serve(sender_address: String, staking_canister: CanisterId) -> u64 {
    repositories::staking_record::total_staking(sender_address, staking_canister)
}
