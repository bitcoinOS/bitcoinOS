// use crate::{domain::UserInfo, error::Error, repositories};
// use candid::Principal;
// use wallet::utils::ic_time;
// pub fn serve(user_id: Principal) -> Result<bool, Error> {
//     let user_opt = repositories::user::get_user_info(user_id);
//     if let Some(u) = user_opt {
//         let now = ic_time();
//         let reward_time = u.last_reward_at;
//         if now - reward_time > 24 * 86400 {
//             let new_user = UserInfo {
//                 last_reward_at: now,
//                 updated_at: now,
//                 ..u
//             };
//             let update_res = repositories::user::update_user_info(new_user);
//             update_res
//         } else {
//             Err(Error::RewardTimeIn24Hour { time: reward_time })
//         }
//     } else {
//         Err(Error::UserNotExists { user_id: user_id })
//     }
// }
