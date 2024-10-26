use crate::domain::response::BoxRecordResponse;
use crate::repositories::box_record;
use crate::repositories::box_reward;
use crate::repositories::invite_reward;
use crate::repositories::reward_record;
use candid::Principal;
use wallet::utils::ic_time;

use crate::{domain::RewardRecord, error::Error};

pub fn serve(user_id: Principal, box_id: u64) -> Result<BoxRecordResponse, Error> {
    let user_reward = reward_record::get_user_reward(user_id);
    if let Some(ur) = user_reward {
        let box_open_res = box_record::open_one_box(user_id, box_id);
        match box_open_res {
            Ok(br) => {
                let t = ic_time();
                box_reward::add_box_reward(br.clone(), 1);
                let new_reward = RewardRecord {
                    total_point: ur.total_point + br.point,
                    update_time: t,
                    ..ur
                };
                reward_record::save_reward(new_reward);
                invite_reward::update_invite_user_reward(user_id, br.point);
                let box_record_res = BoxRecordResponse {
                    open_count: 1,
                    user_id,
                    og_count: 0,
                    fund_count: 0,
                    boost_card_count: 0,
                    box_point: br.point,
                };
                Ok(box_record_res)
            }
            Err(e) => Err(e),
        }
    } else {
        Err(Error::UserUnInit(user_id))
    }
}
