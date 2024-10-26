use wallet::utils::get_time_for_day;

use crate::{domain::UserStat, repositories};

pub fn serve() -> Vec<UserStat> {
    let now_user_count = repositories::reward_record::get_user_reward_count();
    let now_day = get_time_for_day();
    let mut user_stats = repositories::user_stat::get_user_stat();
    user_stats.push(UserStat {
        day: now_day,
        user_count: now_user_count as u128,
    });
    user_stats
}
