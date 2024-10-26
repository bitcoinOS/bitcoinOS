use std::borrow::BorrowMut;

use wallet::utils::get_time_for_day;

use crate::{context::STATE, domain::UserStat};

pub fn get_user_stat() -> Vec<UserStat> {
    let mut user_stats: Vec<UserStat> =
        STATE.with_borrow(|s| s.user_stat.iter().map(|(_, v)| v).collect());
    user_stats.sort_by(|a, b| a.day.cmp(&b.day));
    user_stats
}
pub fn update_user_stat(now_day_user_count: u128) {
    STATE.with_borrow_mut(|s| {
        let now_day = get_time_for_day();
        let users = s.user_stat.borrow_mut();
        if users.len() > 0 {
            let max_day_stat = users.get(&7u64).unwrap();
            // need update
            if max_day_stat.day < now_day {
                for i in 1..5u64 {
                    let next_day_users = users.get(&(i + 1)).unwrap();
                    users.insert(i, next_day_users);
                }
            }
            let now_day_users = UserStat {
                day: now_day,
                user_count: now_day_user_count,
            };
            users.insert(6u64, now_day_users);
        }
    })
}

pub fn init_user_stat(user_stat: Vec<UserStat>) {
    STATE.with_borrow_mut(|s| {
        let users = s.user_stat.borrow_mut();
        for (i, v) in user_stat.iter().enumerate() {
            users.insert(7 - i as u64, v.to_owned());
        }
    });
}

pub fn get_user_count() -> u64 {
    STATE.with_borrow(|s| s.user_stat.len())
}
