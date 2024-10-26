use crate::{
    context::STATE,
    domain::{BoxRewardRecord, InviteRewardRecord, RewardRecord, StakeRewardRecord},
};
use candid::Principal;
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;

use wallet::{domain::user::UserType, utils::ic_time};
pub fn init(user_id: Principal, user_type: UserType, network: BitcoinNetwork) {
    STATE.with_borrow_mut(|s| {
        let t = ic_time();
        // ic_cdk::print("init 2");
        let reward_records = &mut s.reward_record;
        if !reward_records.contains_key(&user_id) {
            // ic_cdk::print("init 3");
            let reward_record = RewardRecord {
                user_id: user_id,
                create_time: t,
                update_time: t,
                network: network,
                user_type: user_type,
                ..Default::default()
            };
            reward_records.insert(user_id, reward_record);
        }

        let box_reward_records = &mut s.box_reward;
        if !box_reward_records.contains_key(&user_id) {
            // ic_cdk::print("init 4");
            let box_reward_record = BoxRewardRecord {
                user_id: user_id,
                create_time: t,
                update_time: t,
                network: network,
                ..Default::default()
            };
            box_reward_records.insert(user_id, box_reward_record);
        }

        let invite_reward_records = &mut s.invite_reward;
        if !invite_reward_records.contains_key(&user_id) {
            // ic_cdk::print("init 5");
            let invite_reward_record = InviteRewardRecord {
                user_id: user_id,
                create_time: t,
                update_time: t,
                network: network,
                ..Default::default()
            };
            invite_reward_records.insert(user_id, invite_reward_record);
        }

        let stake_reward_records = &mut s.stake_reward;
        if !stake_reward_records.contains_key(&user_id) {
            // ic_cdk::print("init 6");
            let stake_reward_record = StakeRewardRecord {
                user_id: user_id,
                create_time: t,
                update_time: t,
                network: network,
                ..Default::default()
            };
            stake_reward_records.insert(user_id, stake_reward_record);
        }
    })
}
