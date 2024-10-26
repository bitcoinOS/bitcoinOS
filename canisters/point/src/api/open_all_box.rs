// use std::borrow::BorrowMut;

use crate::domain::response::BoxRecordResponse;
use crate::domain::BoxStatus;
// use crate::constants::BOX_BASE_POINT;
use crate::repositories::{box_record, box_reward, invite_reward};
// use crate::repositories::user_status_record::update_invite_record;
use candid::Principal;
use wallet::utils::ic_time;

use crate::repositories::reward_record::save_reward;
use crate::{
    domain::{BoxRecord, RewardRecord},
    error::Error,
    repositories::reward_record::get_user_reward,
};

use super::get_user_boxes;

pub fn serve(user_id: Principal) -> Result<BoxRecordResponse, Error> {
    let box_res = get_user_boxes::serve(user_id, BoxStatus::Close);
    let mut open_box = 0u64;
    let mut open_point = 0u64;
    let mut og_count = 0u64;
    let mut fund_count = 0u64;
    let mut boost_card_count = 0u64;

    match box_res {
        Some(br) => {
            let user_reward = get_user_reward(user_id);
            if let Some(ur) = user_reward {
                let t = ic_time();
                for sbr in br {
                    let open_box_res = box_record::open_one_box(user_id, sbr.box_id);
                    match open_box_res {
                        Ok(box_record) => {
                            open_box += 1;
                            open_point += box_record.point;
                            og_count += box_record.og_count;
                            fund_count += box_record.fund_count;
                            boost_card_count += box_record.boost_card_count;
                        }
                        Err(_) => {
                            ic_cdk::print(format!(
                                "user:{0}open box:{1} fail",
                                user_id, sbr.box_id
                            ));
                        }
                    }
                }
                let new_reward = RewardRecord {
                    total_point: ur.total_point + open_point,
                    update_time: t,
                    ..ur
                };
                save_reward(new_reward);
                box_reward::add_box_reward(
                    BoxRecord {
                        user_id,
                        point: open_point,
                        og_count,
                        fund_count,
                        boost_card_count,
                        box_status: BoxStatus::Open,
                        ..Default::default()
                    },
                    open_box,
                );
                invite_reward::update_invite_user_reward(user_id, open_point);
                let box_record_res = BoxRecordResponse {
                    open_count: open_box,
                    user_id,
                    og_count: 0,
                    fund_count: 0,
                    boost_card_count: 0,
                    box_point: open_point,
                };
                Ok(box_record_res)
            } else {
                Err(Error::UserUnInit(user_id))
            }
        }
        None => Ok(BoxRecordResponse {
            user_id,
            ..Default::default()
        }),
    }
}
