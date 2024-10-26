// use wallet::utils::ic_time;

// use crate::{
//     domain::{LeaderBoardStatus, RewardRecord, StakeRewardRecord},
//     repositories,
// };

// pub async fn serve() {
//     update_user_point_rank();
//     update_user_stake_rank();
// }

// fn update_user_point_rank() {
//     let mut user_rewards = repositories::reward_record::get_all_user_reward();
//     user_rewards.sort_by(|a, b| b.total_point.cmp(&a.total_point));
//     if !user_rewards.is_empty() {
//         let total_user = user_rewards.len() as u64;
//         let max_point = user_rewards.first().unwrap().total_point;
//         let mut min_point = 0;
//         let mut pre_point = max_point;
//         let mut user_gt_zero = 0u64;
//         let mut pre_rank = 1u64;
//         let mut seq = 0u64;
//         let leader_board_count = repositories::leaderboard::get_leader_board().len() as u64;
//         if max_point > 0 {
//             let t = ic_time();
//             for u in user_rewards {
//                 if u.total_point > 0 {
//                     if u.total_point < pre_point {
//                         pre_rank += 1;
//                         pre_point = u.total_point;
//                     }
//                     user_gt_zero += 1;
//                     min_point = u.total_point;

//                     //更新用户排名
//                     let new_reward_record = RewardRecord {
//                         rank: pre_rank,
//                         update_time: t,
//                         ..u
//                     };
//                     let res = repositories::reward_record::save_reward(new_reward_record.clone());
//                     if res.is_some() {
//                         //更新leaderboard
//                         if pre_rank <= 100 {
//                             seq += 1;
//                             repositories::leaderboard::update_leader_board(seq, new_reward_record);
//                         }
//                     }
//                 } else {
//                     break;
//                 }
//             }
//             //清除多的排名
//             if seq < leader_board_count {
//                 for i in seq + 1..=leader_board_count {
//                     repositories::leaderboard::remove_leader_board(i);
//                 }
//             }
//             //更新全局状态
//             let ls = LeaderBoardStatus {
//                 total_user_gt_zero: user_gt_zero,
//                 update_time: t,
//                 total_user,
//                 min_rank: pre_rank,
//                 max_point,
//                 min_point,
//             };
//             repositories::leaderboard::update_leader_board_status(ls);
//         }
//     }
// }

// fn update_user_stake_rank() {
//     let mut user_stake_rewards = repositories::stake_reward::get_all_user_stake_reward();
//     let user_invite_rewards =  repositories::invite_reward::get_all_user_invite_reward();
//     user_stake_rewards.sort_by(|a, b| b.stake_point.cmp(&a.stake_point));
//     if !user_stake_rewards.is_empty() {
//         // let total_user = user_stake_rewards.len() as u64;
//         let max_point = user_stake_rewards.first().unwrap().stake_point;
//         // let mut min_point = 0;
//         let mut pre_point = max_point;
//         // let mut user_gt_zero = 0u64;
//         let mut pre_rank = 1u64;
//         let mut seq = 0u64;
//         let leader_board_count = repositories::leaderboard::get_stake_leader_board().len() as u64;
//         if max_point > 0 {
//             let t = ic_time();
//             for u in user_stake_rewards {
//                 if u.stake_point > 0 {
//                     if u.stake_point < pre_point {
//                         pre_rank += 1;
//                         pre_point = u.stake_point;
//                     }
//                     // user_gt_zero += 1;
//                     // min_point = u.stake_point;

//                     //更新用户排名
//                     let new_reward_record = StakeRewardRecord {
//                         credit_rank: Some(pre_rank),
//                         rank_credit_point: Some(u.stake_point),
//                         update_time: t,
//                         ..u
//                     };
//                     let res = repositories::stake_reward::save_reward(new_reward_record.clone());
//                     if res.is_some() {
//                         //更新leaderboard
//                         if pre_rank <= 100 {
//                             seq += 1;
//                             repositories::leaderboard::update_stake_leader_board(
//                                 seq,
//                                 new_reward_record,
//                             );
//                         }
//                     }
//                 } else {
//                     break;
//                 }
//             }
//             //清除多的排名
//             if seq < leader_board_count {
//                 for i in seq + 1..=leader_board_count {
//                     repositories::leaderboard::remove_stake_leader_board(i);
//                 }
//             }
//         }
//     }
// }
