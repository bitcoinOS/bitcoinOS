use wallet::utils::{get_time_for_day, sub_day_n_day, time_to_day};

use crate::{domain::UserStat, repositories};

pub fn serve() {
    let user_count = repositories::user_stat::get_user_count();
    if user_count == 0 {
        let now_day = get_time_for_day();
        let mut user_rewards = repositories::reward_record::get_all_user_reward();
        user_rewards.sort_by(|a, b| b.create_time.cmp(&a.create_time));
        let total_user = user_rewards.len() as u128;
        let mut flag_count = 0u128;
        let mut pre_day = sub_day_n_day(now_day, 1);
        let mut user_stat: Vec<UserStat> = vec![];
        let mut flag = 0u8;
        for u in user_rewards {
            let user_create_day = time_to_day(u.create_time);
            if user_create_day < pre_day {
                let pre_count = total_user - flag_count;
                user_stat.push(UserStat {
                    day: pre_day,
                    user_count: pre_count,
                });
                pre_day = sub_day_n_day(pre_day, 1);
                flag = flag + 1;
                if flag >= 6 {
                    break;
                }
            }
            flag_count = flag_count + 1;
        }
        repositories::user_stat::init_user_stat(user_stat);
    }
}
