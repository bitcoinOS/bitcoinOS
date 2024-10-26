use candid::Principal;

use crate::{constants::POINT_DECIMAL, repositories};

pub fn serve(user_id: Principal) -> u64 {
   let point_reward =  repositories::reward_record::get_user_reward(user_id);
   let mut total_point =0;
   if let Some(r) = point_reward{
    total_point +=r.total_point*POINT_DECIMAL;
   }
   // let nft_reward=  repositories::staked_nft_reward::get_user_stake_nft_reward(user_id);
   // if let Some(r) = nft_reward{
   //  total_point +=r.stake_nft_point;
   // }
   let stake_reward= repositories::stake_reward::get_user_stake_reward(user_id);
   if let Some(r) = stake_reward{
    if r.stake_point_update_at.is_some() && r.stake_point_update_at.unwrap() >0{
      total_point +=r.stake_point;
    }else{
      total_point +=r.stake_point*POINT_DECIMAL;
    }
   }
   total_point/1000
}
