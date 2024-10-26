use std::collections::HashMap;

use crate::{
    constants,
    context::STATE,
    domain::{InviteRewardRecord, InviteStaus, RewardRecord},
    error::Error,
};
use candid::Principal;
use wallet::utils::ic_time;

pub fn get_user_invite_reward(user_id: Principal) -> Option<InviteRewardRecord> {
    STATE.with_borrow(|s| s.invite_reward.get(&user_id))
}

pub fn get_all_user_invite_reward() -> HashMap<Principal, InviteRewardRecord> {
    STATE.with_borrow(|s| {
        let mut user_invite_map: HashMap<Principal, InviteRewardRecord> = HashMap::new();
        for (k, v) in s.invite_reward.iter() {
            user_invite_map.insert(k, v);
        }
        user_invite_map
    })
}

// pub fn update_invite_reward(
//     invite_reward_record: InviteRewardRecord,
// ) -> Option<InviteRewardRecord> {
//     STATE.with_borrow_mut(|s| {
//         let records = &mut s.invite_reward;
//         records.insert(invite_reward_record.user_id, invite_reward_record)
//     })
// }

pub fn add_invite(invited_user_id: Principal, invite_user_id: Principal) -> bool {
    STATE.with_borrow_mut(|s| {
        let records = &mut s.invite_reward;
        let invited_user_reward = records.get(&invited_user_id);
        match invited_user_reward {
            Some(ieur) => {
                let t = ic_time();
                if ieur.invite_status == InviteStaus::UnInvited {
                    let invite_reward_record = InviteRewardRecord {
                        invite_status: InviteStaus::Init,
                        invite_user_id: Some(invite_user_id),
                        update_time: t,
                        ..ieur
                    };
                    let res = records.insert(invited_user_id, invite_reward_record);
                    if res.is_some() {
                        let invite_user_reward = records.get(&invite_user_id);
                        if let Some(iur) = invite_user_reward {
                            let new_iur = InviteRewardRecord {
                                total_invite_count: iur.total_invite_count + 1,
                                update_time: t,
                                ..iur
                            };
                            let res = records.insert(invite_user_id, new_iur);
                            if res.is_some() {
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            None => false,
        }
    })
}

pub fn update_invite_user_reward(invited_user_id: Principal, point: u64) {
    STATE.with_borrow_mut(|s| {
        // ic_cdk::print(" update_invite_user_reward in time period 1\n");
        let invite_record_store = &mut s.invite_reward;
        let invite_reward = invite_record_store.get(&invited_user_id);
        let config = s.config_setting.get();
        if let Some(ur) = invite_reward {
            // ic_cdk::print(" update_invite_user_reward in time period 2\n");
            let t = ic_time();
            if ur.invite_status == InviteStaus::Confirmed {
                let invite_point = point / config.invite_point_rate;

                let invite_user_id = ur.invite_user_id;
                if let Some(iui) = invite_user_id {
                    let invite_user_invite_reward = invite_record_store.get(&iui);
                    if let Some(iuir) = invite_user_invite_reward {
                        let new_invite_reward = InviteRewardRecord {
                            invited_points: iuir.invited_points + invite_point,
                            update_time: t,
                            ..iuir
                        };
                        invite_record_store.insert(iui, new_invite_reward);
                    }

                    let user_reward_store = &mut s.reward_record;

                    let invite_user_reward = user_reward_store.get(&iui);
                    if let Some(iur) = invite_user_reward {
                        let new_user_reward = RewardRecord {
                            total_point: iur.total_point + invite_point,
                            update_time: t,
                            ..iur
                        };
                        user_reward_store.insert(iui, new_user_reward);
                    }
                }
            }
        }
    });
}

pub fn confirm_invite_status(invited_user_id: Principal, point: u64) -> Result<bool, Error> {
    STATE.with_borrow_mut(|s| {
        let invite_record_store = &mut s.invite_reward;
        let invite_reward = invite_record_store.get(&invited_user_id);
        if let Some(ur) = invite_reward {
            // ic_cdk::print(" confirm_invite_status in time period 2\n");
            let t = ic_time();
            if ur.invite_status == InviteStaus::Init {
                let new_ur = InviteRewardRecord {
                    invite_status: InviteStaus::Confirmed,
                    update_time: t,
                    ..ur
                };
                let invite_res = invite_record_store.insert(invited_user_id, new_ur);
                if invite_res.is_some() {
                    // ic_cdk::print(" confirm_invite_status in time period 3\n");
                    let invite_user_id = ur.invite_user_id;

                    let invite_point = point / constants::POINT_DECIMAL;
                    // ic_cdk::print(" confirm_invite_status in time period 4\n");

                    if let Some(iui) = invite_user_id {
                        ic_cdk::print(" confirm_invite_status in time period 5\n");
                        //add invite box
                        let invite_user_invite_reward = invite_record_store.get(&iui);
                        if let Some(iuir) = invite_user_invite_reward {
                            let new_invite_reward = InviteRewardRecord {
                                invited_points: iuir.invited_points + invite_point,
                                avalable_invite_count: iuir.avalable_invite_count + 1,
                                update_time: t,
                                ..iuir
                            };
                            let invite_res = invite_record_store.insert(iui, new_invite_reward);
                            if invite_res.is_some() {
                                let user_reward_store = &mut s.reward_record;
                                let invite_user_reward = user_reward_store.get(&iui);
                                if let Some(iur) = invite_user_reward {
                                    let new_user_reward = RewardRecord {
                                        total_point: iur.total_point + invite_point,
                                        update_time: t,
                                        ..iur
                                    };
                                    user_reward_store.insert(iui, new_user_reward);
                                }
                            }
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    })
}

// pub fn update_statke_reward(user_id: Principal, stake_amount: u64) {
//     STATE.with_borrow_mut(|s| {
//         let records = &mut s.reward_record;
//         let user_reward = records.get(&user_id);
//         let user_status_record_store = &mut s.user_status_record;
//         let user_status_record = user_status_record_store.get(&user_id).unwrap();
//         let point = stake_amount / 10000 + 1;
//         match user_reward {
//             Some(mut u) => {
//                 let t = ic_time();
//                 *u.total_point.borrow_mut() += point;
//                 *u.stake_amount.borrow_mut() = stake_amount;
//                 *u.last_stake_reward_at.borrow_mut() = t;
//                 *u.update_time.borrow_mut() = t;
//                 if user_status_record.invite_status == InviteStaus::Confirmed {
//                     let invite_user_reward = records.get(&user_status_record.invite_user_id);
//                     match invite_user_reward {
//                         Some(mut iu) => {
//                             *iu.total_point.borrow_mut() += point / 10;
//                             *iu.invited_points.borrow_mut() += point / 10;
//                         }
//                         None => {}
//                     }
//                 }
//             }
//             None => {}
//         }
//     });
// }

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
