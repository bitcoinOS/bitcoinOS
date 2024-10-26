// use candid::Principal;

// use crate::{
//     context::STATE,
//     domain::{BoxKey, BoxRecord, BoxRewardDetail, BoxStaus},
//     error::Error,
// };

// use super::{reward_record, user_status_record};
// use std::ops::RangeBounds;

// pub fn add_box_reward(box_reward: BoxRewardDetail) -> Result<Option<BoxRewardDetail>, Error> {
//     STATE.with_borrow_mut(|s| {
//         let records = &mut s.box_reward_record;
//         let box_key = BoxKey {
//             user_id: box_reward.user_id,
//             box_id: box_reward.box_id,
//         };
//         if records.contains_key(&box_key) {
//             Err(Error::BoxRewardHasExists(
//                 box_reward.user_id,
//                 box_reward.box_id,
//             ))
//         } else {
//             let r = records.insert(box_key, box_reward.to_owned());
//             Ok(r)
//         }
//     })
// }

// // pub fn  get_box(user_id:Principal,box_id:u64)->Option<BoxRecord>{
// //     STATE.with_borrow(|s| {
// //         let records = & s.box_record;
// //         let box_key = BoxKey{
// //             user_id:user_id,
// //             box_id:box_id
// //         };
// //          records.get(&box_key)
// //     })
// // }

// // pub fn  update_box(box_record:BoxRecord)->Result<Option<BoxRecord>,Error>{
// //     STATE.with_borrow_mut(|s| {
// //         let records = &mut s.box_record;
// //         let box_key = BoxKey{
// //             user_id:box_record.user_id,
// //             box_id:box_record.box_id
// //         };
// //         if !records.contains_key(&box_key){
// //             Err(Error::BoxNotExists(box_record.user_id,box_record.box_id))
// //         }else{
// //             let r = records.insert(box_key, box_record.to_owned());
// //             Ok(r)
// //         }
// //     })
// // }

// pub fn get_user_box_rewards(user_id: Principal) -> Option<Vec<BoxRewardDetail>> {
//     let user_status = user_status_record::get_user_status_record(user_id);
//     match user_status {
//         Some(r) => {
//             let max_box_id = r.max_box_id;
//             let box_rewards = find_info_by_owner(user_id, max_box_id);
//             Some(box_rewards)
//         }
//         None => None,
//     }
// }

// pub(crate) fn find_info_by_owner(owner: Principal, max_box_id: u64) -> Vec<BoxRewardDetail> {
//     STATE.with_borrow(|s| {
//         s.box_reward_record
//             .range(range_owner_filter(owner, max_box_id))
//             .map(|(_, info)| info)
//             .collect()
//     })
// }

// fn range_owner_filter(owner: Principal, max_box_id: u64) -> impl RangeBounds<BoxKey> {
//     let start = BoxKey {
//         user_id: owner,
//         box_id: 0,
//     };

//     let end = BoxKey {
//         user_id: owner,
//         box_id: max_box_id,
//     };

//     start..end
// }

// // pub fn get_user_unopen_boxes(user_id:Principal)->Option<Vec<BoxRecord>>{
// //     let user_reward = reward_record::get_user_reward(user_id);
// //     match user_reward {
// //         Some(r) =>{
// //                 let max_box_id = r.max_box_id;
// //                 let boxes = find_unopen_by_owner(user_id,max_box_id);
// //                 Some(boxes)
// //         }
// //         None=>{
// //             None
// //         }
// //     }
// // }

// // pub(crate) fn find_unopen_by_owner(owner: Principal,max_box_id:u64) -> Vec<BoxRecord> {
// //     STATE.with_borrow(|s| {
// //         s.box_record
// //             .range(range_owner_filter(owner,max_box_id))
// //             .map(|(_, info)| info)
// //             .filter(|b| b.box_status == BoxStaus::Close)
// //             .collect()
// //     })
// // }
