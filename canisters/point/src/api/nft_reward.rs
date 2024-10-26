
use crate::{
    constants::POINT_DECIMAL, domain::{request::NftRecordRequest, NftRewardRecord}, repositories::{self, config_setting, staked_nft_reward}
};
use candid::Principal;
use wallet::utils::ic_time;

pub fn get_all_user_stake_nft_reward() -> Vec<NftRewardRecord> {
    staked_nft_reward::get_all_user_stake_nft_reward()
}

pub fn get_user_stake_nft_reward(user_id: Principal) -> Option<NftRewardRecord> {
    let nft_reward = staked_nft_reward::get_user_stake_nft_reward(user_id);
    if let Some(r) = nft_reward{
        let new_reward = NftRewardRecord{
            stake_nft_point: r.stake_nft_point / 1000,
            ..r
        };
        Some(new_reward)
    }else{  
        None
    }
}

pub fn update_all_nft_reward(nft_requests: Vec<NftRecordRequest>) {
    for v in nft_requests {
        update_nft_reward(v);
    }
}
pub fn update_nft_reward(nft_request: NftRecordRequest) -> bool {
    let time = ic_time();
    let metadata = repositories::metadata::get_metadata();
    let nft_reward = staked_nft_reward::get_user_stake_nft_reward(nft_request.user_id);
    let config = config_setting::get_config_setting();
    let boost_rate = get_nft_boost_rate(nft_request.nft_count);
    if let Some(nr) = nft_reward {
        let duration_sec = (time - nr.stake_nft_point_update_at) / 1000000000;
        let point = nft_request.nft_count * nft_request.nft_price * boost_rate
            / config.sat_per_point
            * duration_sec
            / 100;
        let new_nft_reward = NftRewardRecord {
            staked_nft_count: nft_request.nft_count,
            boost_rate: boost_rate,
            stake_nft_point: nr.stake_nft_point + point,
            stake_nft_point_update_at: time,
            update_time: time,
            ..nr
        };
        staked_nft_reward::save_reward(new_nft_reward);
        true
    } else {
        let point =
            nft_request.nft_count * nft_request.nft_price * boost_rate / config.sat_per_point * POINT_DECIMAL/100; //1000/100
        let new_nft_reward = NftRewardRecord {
            user_id: nft_request.user_id,
            network: metadata.network,
            staked_nft_count: nft_request.nft_count,
            boost_rate: boost_rate,
            stake_nft_point: point,
            stake_nft_point_update_at: time,
            update_time: time,
            create_time: time,
        };
        staked_nft_reward::save_reward(new_nft_reward);
        true
    }
}

fn get_nft_boost_rate(nft_count: u64) -> u64 {
     nft_count*10+100
}
