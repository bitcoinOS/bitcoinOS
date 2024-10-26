use crate::{domain::RewardRecord, repositories};

pub fn serve() -> Vec<RewardRecord> {
    repositories::reward_record::get_all_user_reward()
    
}
