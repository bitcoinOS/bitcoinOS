use candid::Principal;

use crate::{domain::BoxRewardRecord, repositories};

pub fn serve(user_id: Principal) -> Option<BoxRewardRecord> {
    repositories::box_reward::get_user_box_reward(user_id)
}
