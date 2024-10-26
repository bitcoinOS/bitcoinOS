use candid::Principal;

use crate::{domain::InviteRewardRecord, repositories};

pub fn serve(user_id: Principal) -> Option<InviteRewardRecord> {
    repositories::invite_reward::get_user_invite_reward(user_id)
}
