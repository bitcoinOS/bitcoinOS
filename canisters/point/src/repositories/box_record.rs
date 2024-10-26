use candid::Principal;
use wallet::{
    constants,
    domain::reward::{self, RewardMode, RewardType},
    utils::ic_time,
};

use crate::{
    context::STATE,
    domain::{BoxKey, BoxRecord, BoxRewardRecord, BoxStatus},
    error::Error,
};

use super::box_reward;
use std::ops::RangeBounds;

pub fn add_login_box(user_id: Principal) -> Result<Option<BoxRecord>, Error> {
    add_box(user_id, RewardType::Login, RewardMode::Random)
}

pub fn add_invite_box(user_id: Principal) -> Result<Option<BoxRecord>, Error> {
    add_box(user_id, RewardType::Invite, RewardMode::Random)
}

pub fn add_stake_box(user_id: Principal) -> Result<Option<BoxRecord>, Error> {
    add_box(user_id, RewardType::Stake, RewardMode::Random)
}

pub fn add_red_packet_box(
    user_id: Principal,
    reward_mode: RewardMode,
) -> Result<Option<BoxRecord>, Error> {
    add_box(user_id, RewardType::RedPacket, reward_mode)
}

pub fn add_box(
    user_id: Principal,
    box_type: RewardType,
    reward_mode: RewardMode,
) -> Result<Option<BoxRecord>, Error> {
    STATE.with_borrow_mut(|s| {
        let box_reward_store = &mut s.box_reward;
        let box_reward_record = box_reward_store.get(&user_id);
        // ic_cdk::print("add box 1");
        if let Some(br) = box_reward_record {
            // ic_cdk::print("add box 2");
            let max_id = br.max_box_id;
            let records = &mut s.box_record;
            let box_key = BoxKey {
                user_id,
                box_id: max_id,
            };
            if records.contains_key(&box_key) {
                Err(Error::BoxHasExists(user_id, max_id))
            } else {
                let t = ic_time();
                let box_record = BoxRecord {
                    user_id,
                    box_id: max_id,
                    box_status: BoxStatus::Close,
                    box_type,
                    point: 0,
                    og_count: 0,
                    fund_count: 0,
                    boost_card_count: 0,
                    create_time: t,
                    open_time: 0,
                    reward_mode: Some(reward_mode),
                };
                let r: Option<BoxRecord> = records.insert(box_key, box_record.to_owned());
                let new_box_reward = BoxRewardRecord {
                    max_box_id: max_id + 1,
                    total_box_count: br.total_box_count + 1,
                    unopen_box_count: br.unopen_box_count + 1,
                    update_time: t,
                    ..br
                };
                box_reward_store.insert(user_id, new_box_reward);
                Ok(r)
            }
        } else {
            Err(Error::UserUnInit(user_id))
        }
    })
}

// pub fn get_box(user_id: Principal, box_id: u64) -> Option<BoxRecord> {
//     STATE.with_borrow(|s| {
//         let records = &s.box_record;
//         let box_key = BoxKey {
//             user_id: user_id,
//             box_id: box_id,
//         };
//         records.get(&box_key)
//     })
// }

pub fn open_one_box(user_id: Principal, box_id: u64) -> Result<BoxRecord, Error> {
    STATE.with_borrow_mut(|s| {
        let records = &mut s.box_record;
        let box_key = BoxKey { user_id, box_id };
        let box_record = records.get(&box_key);
        if let Some(br) = box_record {
            if br.box_status == BoxStatus::Close {
                let config = s.config_setting.get();
                let t = ic_time();
                let reward_mode = if let Some(rm) = br.reward_mode {
                    rm
                } else {
                    RewardMode::Random
                };
                let point = if reward_mode == RewardMode::Random {
                    config.base_point_per_box
                        + (t + box_id * box_id + 3 * box_id + 5 * (box_id % 10))
                            % config.max_rand_box_point
                } else {
                    crate::constants::FIXED_POINT
                };

                let new_box = BoxRecord {
                    box_status: BoxStatus::Open,
                    point,
                    open_time: t,
                    reward_mode: Some(reward_mode),
                    ..br
                };
                let open_res = records.insert(box_key, new_box.clone());
                if open_res.is_some() {
                    Ok(new_box)
                } else {
                    Err(Error::BoxOpenError(user_id, box_id))
                }
            } else {
                Err(Error::BoxHasOpen(user_id, box_id))
            }
        } else {
            Err(Error::BoxNotExists(user_id, box_id))
        }
    })
}

// pub fn update_box(box_record: BoxRecord) -> Result<Option<BoxRecord>, Error> {
//     STATE.with_borrow_mut(|s| {
//         let records = &mut s.box_record;
//         let box_key = BoxKey {
//             user_id: box_record.user_id,
//             box_id: box_record.box_id,
//         };
//         if !records.contains_key(&box_key) {
//             Err(Error::BoxNotExists(box_record.user_id, box_record.box_id))
//         } else {
//             let r = records.insert(box_key, box_record.to_owned());
//             Ok(r)
//         }
//     })
// }

// pub fn get_user_boxes(user_id: Principal) -> Option<Vec<BoxRecord>> {
//     let user_box_reward_opt = box_reward::get_user_box_reward(user_id);
//     if let Some(ubr) = user_box_reward_opt {
//         let max_box_id = ubr.max_box_id;
//         let boxes = find_info_by_owner(user_id, max_box_id);
//         Some(boxes)
//     } else {
//         None
//     }
// }

// fn find_info_by_owner(owner: Principal, max_box_id: u64) -> Vec<BoxRecord> {
//     STATE.with_borrow(|s| {
//         s.box_record
//             .range(range_owner_filter(owner, max_box_id))
//             .map(|(_, info)| info)
//             .collect()
//     })
// }

fn range_owner_filter(owner: Principal, max_box_id: u64) -> impl RangeBounds<BoxKey> {
    let start = BoxKey {
        user_id: owner,
        box_id: 0,
    };

    let end = BoxKey {
        user_id: owner,
        box_id: max_box_id + 1,
    };

    start..end
}

pub fn get_user_boxes_by_status(user_id: Principal, status: BoxStatus) -> Option<Vec<BoxRecord>> {
    let user_box_reward_opt = box_reward::get_user_box_reward(user_id);
    if let Some(ubr) = user_box_reward_opt {
        let max_box_id = ubr.max_box_id;
        let boxes = find_by_status(user_id, max_box_id, status);
        Some(boxes)
    } else {
        None
    }
}

pub(crate) fn find_by_status(
    owner: Principal,
    max_box_id: u64,
    status: BoxStatus,
) -> Vec<BoxRecord> {
    STATE.with_borrow(|s| {
        s.box_record
            .range(range_owner_filter(owner, max_box_id))
            .map(|(_, info)| info)
            .filter(|b| b.box_status == status)
            .collect()
    })
}
