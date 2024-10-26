use crate::{
    context::STATE,
    domain::{BoxRecord, BoxRewardRecord, BoxStatus},
};
use candid::Principal;
use wallet::utils::ic_time;

pub fn get_user_box_reward(user_id: Principal) -> Option<BoxRewardRecord> {
    STATE.with_borrow(|s| s.box_reward.get(&user_id))
}

// pub fn save_box_reward(box_reward_record: BoxRewardRecord) -> Option<BoxRewardRecord> {
//     STATE.with_borrow_mut(|s| {
//         let records = &mut s.box_reward;
//         records.insert(box_reward_record.user_id, box_reward_record)
//     })
// }

pub fn add_box_reward(box_record: BoxRecord, count: u64) {
    STATE.with_borrow_mut(|s| {
        let records = &mut s.box_reward;
        let user_id = box_record.user_id;
        let box_reward = records.get(&user_id);
        if let Some(br) = box_reward {
            if box_record.box_status == BoxStatus::Open {
                let t = ic_time();
                let new_box_reward = BoxRewardRecord {
                    unopen_box_count: br.unopen_box_count - count,
                    box_point: br.box_point + box_record.point,
                    og_count: br.og_count + box_record.og_count,
                    fund_count: br.fund_count + box_record.fund_count,
                    boost_card_count: br.boost_card_count + box_record.boost_card_count,
                    update_time: t,
                    ..br
                };
                let _ = records.insert(user_id, new_box_reward);
            }
        }
    })
}

// pub fn add_reward(user_id:Principal,box_record: BoxRecord) {
//     STATE.with_borrow_mut(|s| {
//         let records = &mut s.box_reward;
//         let user_id = box_record.user_id;
//         let box_reward = records.get(&user_id);
//         if let Some(br) = box_reward {
//             if box_record.box_status == BoxStatus::Open {
//                 let t = ic_time();
//                 let new_box_reward = BoxRewardRecord {
//                     unopen_box_count: br.unopen_box_count - 1,
//                     box_point: br.box_point + box_record.point,
//                     og_count: br.og_count + box_record.og_count,
//                     fund_count: br.fund_count + box_record.fund_count,
//                     boost_card_count: br.boost_card_count + box_record.boost_card_count,
//                     update_time: t,
//                     ..br
//                 };
//                 let _ = records.insert(user_id, new_box_reward);
//             }
//         }
//     })
// }

// pub fn update_invite_user_reward(invited_user_id: Principal, point: u64) {
//     STATE.with_borrow_mut(|s| {
//         let user_status_store = &s.user_status_record;
//         let user_status_record = user_status_store.get(&invited_user_id);
//         if let Some(ur) = user_status_record {
//             if ur.invite_status == InviteStaus::Confirmed {
//                 let invite_user_id = ur.invite_user_id;
//                 let user_reward_store = &mut s.reward_record;
//                 let invite_user_reward = user_reward_store.get(&invite_user_id);
//                 if let Some(mut iur) = invite_user_reward {
//                     *iur.total_point.borrow_mut() += point;
//                     *iur.invited_points.borrow_mut() += point;
//                     *iur.update_time.borrow_mut() += ic_time();
//                 }
//             }
//         };
//     });

// let invite_res = get_user_status_record(invited_user_id);
// match invite_res {
//     Some(i) => {
//         if i.invite_status == InviteStaus::Confirmed {
//             let user_id = i.invite_user_id;
//             let user_reward_res = get_user_reward(user_id);
//             match user_reward_res {
//                 Some(u) => {
//                     let new_reward = RewardRecord {
//                         total_point: u.total_point + (point / 10) as u64,
//                         update_time: ic_time(),
//                         ..u
//                     };
//                     save_reward(new_reward);
//                 }
//                 None => {}
//             };
//         }
//     }
//     None => {}
// }
// }

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
