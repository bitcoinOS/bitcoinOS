use ic_cdk::api::management_canister::{bitcoin::Satoshi, main::CanisterId};

use crate::repositories;

pub(super) fn serve(staking_canister: CanisterId) -> Satoshi {
    repositories::staking_record::total_staking_by_pool(staking_canister)
}
