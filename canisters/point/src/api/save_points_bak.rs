use candid::Principal;
use wallet::constants::BOOST_RATE;

use crate::constants::REWARD_PERIOD;

use crate::error::StakingError;
use crate::repositories::invite_reward::confirm_invite_status;
use crate::repositories::metadata::get_metadata;

use crate::repositories::{box_record, config_setting, invite_reward, stake_reward};
use wallet::domain::staking::StakingPoolInfo;

// use ic_cdk::api::management_canister::main::CanisterId;
use std::collections::HashMap;
use wallet::domain::staking::{StakingRecord, StakingStatus};
use wallet::utils::{get_siwb_principal, ic_time, is_anonymous};
// # 获取质押池子列表
// # 遍历池子里的质押记录
// # 计算每个人当前的总质押余额
// # 计算每个人的积分
pub async fn get_stake_pools(os_canister: Principal) -> Vec<StakingPoolInfo> {
    let resp: Result<(Vec<StakingPoolInfo>,), _> =
        ic_cdk::call(os_canister, "list_staking_pool", ((),)).await;
    resp.map(|b| b.0).expect("get stake pool error")
}

// pub async fn confirm_stake_record(pool: CanisterId) {
//     let _: Result<(bool,), _> = ic_cdk::call(pool, "confirm_staking_record", ((),)).await;
// }

pub async fn serve() {
    // ic_cdk::print(" point in time period 1\n");

    let metadata = get_metadata();
    let stake_pools = get_stake_pools(metadata.os_canister).await;
    let siwb_canister = metadata.siwb_canister.unwrap();
    let mut total_stake_point_per_user: HashMap<Principal, u64> = HashMap::new();
    // let mut stake_wallet_map: HashMap<String, Principal> = HashMap::new();
    ic_cdk::print(" point in time period 2\n");
    for p in stake_pools {
        ic_cdk::print(" point in time period 3\n");
        let _: Result<(bool,), _> =
            ic_cdk::call(p.staking_pool_canister, "confirm_staking_record", ((),)).await;
        let resp: Result<(Result<Vec<StakingRecord>, StakingError>,), _> =
            ic_cdk::call(p.staking_pool_canister, "list_staking", ((),)).await;
        let res = resp
            .map(|b| match b.0 {
                Ok(v) => Some(v),
                _ => None,
            })
            .expect("get stake error");
        let boost_rate = if let Some(b) = p.boost_rate {
            b
        } else {
            BOOST_RATE
        };    

        if let Some(sr) = res {
            for s in sr {
                // ic_cdk::print(format!("in stake record {s:?}\n"));
                let sender = s.sender;
                let wallet = s.sender_address;
                let sender_user = if is_anonymous(sender) {
                    let sender_principal = get_siwb_principal(siwb_canister, wallet.clone()).await;
                    sender_principal
                } else {
                    sender
                };
                if !is_anonymous(sender_user) {
                    if s.status == StakingStatus::Confirmed {
                        if let Some(ammout) = total_stake_point_per_user.get(&sender_user) {
                            total_stake_point_per_user.insert(sender_user, ammout + s.sent_amount * boost_rate);
                        } else {
                            total_stake_point_per_user.insert(sender_user, s.sent_amount * boost_rate);
                        }
                    }
                }
            }
        }
    }
    let config = config_setting::get_config_setting();
    for (user_id, sat) in total_stake_point_per_user {
        let point = sat / config.sat_per_point / 100 + 1;
        // let user_reward = get_user_reward(user_id);
        // ic_cdk::print(" point in time period 4\n");
        let t = ic_time();
        let stake_reward = stake_reward::get_user_stake_reward(user_id);
        if let Some(sr) = stake_reward {
            // ic_cdk::print(" point in time period 5\n");
            let mut last_stake_at = sr.last_stake_reward_at;
            //TODO  add redeem？  REWARD_PERIOD
            if (t - last_stake_at) >= REWARD_PERIOD && sat > sr.stake_amount {
                // ic_cdk::print(" point in time period 6\n");
                //增加质押奖励
                let add_res = box_record::add_stake_box(user_id);
                if add_res.is_ok() {
                    // ic_cdk::print(" point in time period 7\n");
                    last_stake_at = t;
                }
            }
            //confirm invite
            // invite_reward::confirm_invite_status(user_id, point);
            // update invite user reward
            invite_reward::update_invite_user_reward(user_id, point);
            //update user reward
            let stake_res =
                stake_reward::update_user_statke_reward(user_id, sat, point, last_stake_at);
            if stake_res.is_some() {
                let confirm_res = confirm_invite_status(user_id, point);
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
        }
        // }
    }
}
