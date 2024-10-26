use candid::Principal;

use crate::repositories;

pub(super) fn serve(owner: &Principal) -> Vec<Principal> {
    repositories::staking_record::my_staked_pools(owner)
}
