use candid::Principal;
use wallet::utils::ic_time;

use crate::{
    constants::{POINT_DECIMAL, REWARD_PERIOD},
    domain::StakeRewardRecord,
    repositories::{self, box_record, config_setting, invite_reward},
};
pub fn serve(user_id: Principal, sat: u64) -> bool {
    //增加箱子
    //增加积分
    let  now = ic_time();
    let user_stake_reward = repositories::stake_reward::get_user_stake_reward(user_id);
    if let Some(s) = user_stake_reward {
        let mut last_reward_ammout = if let Some(u) = s.last_stake_ammount{
            u
        }else{
            s.stake_amount
        };
        let mut last_stake_reward_time = s.last_stake_reward_at;
        if now - last_stake_reward_time > REWARD_PERIOD && sat > last_reward_ammout {
            let box_res = repositories::box_record::add_stake_box(user_id);
            if box_res.is_ok() {
                last_reward_ammout = sat;
                last_stake_reward_time = now;
            }
        }
        let config = config_setting::get_config_setting();
        let mut stake_point_update_at =  if let Some(u) = s.stake_point_update_at{
            u
        }else{
             0
        };
        let mut incr_point = 0;
        let mut new_point = 0;
        if stake_point_update_at == 0 {
            new_point = s.stake_point * POINT_DECIMAL;
            if s.last_stake_reward_at > 0 {
                stake_point_update_at = s.last_stake_reward_at;
            }
        } else {
            new_point = s.stake_point;
        }
        if stake_point_update_at > 0 {
            let duration_sec = (now - stake_point_update_at) / 1000000000;
            incr_point = sat / config.sat_per_point * duration_sec;
        } else {
            incr_point = sat / config.sat_per_point * POINT_DECIMAL;
        }
        let new_usr = StakeRewardRecord {
            stake_amount: sat,
            stake_point: new_point + incr_point,
            last_stake_reward_at: last_stake_reward_time,
            last_stake_ammount: Some(last_reward_ammout),
            stake_point_update_at: Some(now),
            update_time: now,
            ..s
        };
        let stake_reward_res = repositories::stake_reward::save_reward(new_usr);
        invite_reward::update_invite_user_reward(user_id, incr_point / POINT_DECIMAL);
        if stake_reward_res.is_some() {
            let confirm_res =
                invite_reward::confirm_invite_status(user_id, incr_point / POINT_DECIMAL);
            match confirm_res {
                Ok(b) => {
                    if b {
                        let invite_record = invite_reward::get_user_invite_reward(user_id);
                        if let Some(ir) = invite_record {
                            let _ = box_record::add_invite_box(ir.invite_user_id.unwrap());
                        }
                    }
                }
                Err(_e) => {}
            }
        }
        true
    } else {
        false
    }
}
