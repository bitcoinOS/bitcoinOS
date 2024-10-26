use crate::{
    context::STATE,
    domain::{RewardRecord, StakeRewardRecord},
};
use candid::Principal;

use wallet::utils::ic_time;

pub fn get_all_user_stake_reward() -> Vec<StakeRewardRecord> {
    STATE.with_borrow(|s| s.stake_reward.iter().map(|(_, w)| w).collect())
}

pub fn get_user_stake_reward(user_id: Principal) -> Option<StakeRewardRecord> {
    STATE.with_borrow(|s| s.stake_reward.get(&user_id))
}

pub fn save_reward(reward_record: StakeRewardRecord) -> Option<StakeRewardRecord> {
    STATE.with_borrow_mut(|s| {
        let records = &mut s.stake_reward;
        records.insert(reward_record.user_id, reward_record)
    })
}

pub fn update_user_statke_reward(
    user_id: Principal,
    stake_amount: u64,
    point: u64,
    last_stake_at: u64,
) -> Option<StakeRewardRecord> {
    STATE.with_borrow_mut(|s| {
        // let config = s.config_setting.get();
        // let point = stake_amount / config.sat_per_point + 1;
        let t = ic_time();
        // ic_cdk::print(" update_user_statke_reward in time period 1\n");
        // user total reward
        // let records = &mut s.reward_record;
        // let user_reward = records.get(&user_id);
        // if let Some(ur) = user_reward {
        //     // ic_cdk::print(" update_user_statke_reward in time period 2\n");
        //     let new_user_reward = RewardRecord {
        //         total_point: ur.total_point + point,
        //         update_time: t,
        //         ..ur
        //     };
        //     records.insert(user_id, new_user_reward);
        // };
        // ic_cdk::print(" update_user_statke_reward in time period 3\n");
        // stake reward
        let stake_record_store = &mut s.stake_reward;
        let user_stake_record = stake_record_store.get(&user_id);
        if let Some(usr) = user_stake_record {
            let new_usr = StakeRewardRecord {
                stake_amount: stake_amount,
                stake_point: usr.stake_point + point,
                last_stake_reward_at: last_stake_at,
                update_time: t,
                ..usr
            };
            let stake_res = stake_record_store.insert(user_id, new_usr);
            stake_res
            // if stake_res.is_some() {
            //     ic_cdk::print(" update_user_statke_reward in time period 4\n");
            //     confirm_invite_status(user_id, point);
            // }
        } else {
            None
        }
    })
}

// pub fn update_last_reward(user_id: Principal, t: u64) {
//     STATE.with_borrow_mut(|s| {
//         let records = &mut s.reward_record;
//         let user_status = records.get(&user_id);
//         match user_status {
//             Some(mut u) => {
//                 *u.last_stake_reward_at.borrow_mut() = t;
//                 *u.update_time.borrow_mut() = t;
//             }
//             None => {}
//         }
//     });
// }
