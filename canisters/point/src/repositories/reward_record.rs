use crate::{context::STATE, domain::RewardRecord};
use candid::Principal;
use wallet::utils::ic_time;

pub fn get_all_user_reward() -> Vec<RewardRecord> {
    STATE.with_borrow(|s| s.reward_record.iter().map(|(_, w)| w).collect())
}

pub fn get_user_reward(user_id: Principal) -> Option<RewardRecord> {
    STATE.with_borrow(|s| s.reward_record.get(&user_id))
}

pub fn save_reward(reward_record: RewardRecord) -> Option<RewardRecord> {
    STATE.with_borrow_mut(|s| {
        let records = &mut s.reward_record;
        records.insert(reward_record.user_id, reward_record)
    })
}

pub fn get_user_reward_count() -> u64 {
    STATE.with_borrow(|s| s.reward_record.len())
}

pub fn add_user_point(user_id: Principal, point: u64) -> bool {
    STATE.with_borrow_mut(|s| {
        let records = &mut s.reward_record;
        let user_reward = records.get(&user_id);
        if let Some(u) = user_reward {
            let new_user_reward = RewardRecord {
                total_point: u.total_point + point,
                update_time: ic_time(),
                ..u
            };
            records.insert(user_id, new_user_reward);
            true
        } else {
            false
        }
    })
}
