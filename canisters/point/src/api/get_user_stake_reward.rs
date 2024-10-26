use candid::Principal;

use crate::{constants::POINT_DECIMAL, domain::StakeRewardRecord, repositories};

pub fn serve(user_id: Principal) -> Option<StakeRewardRecord> {
  let reward =   repositories::stake_reward::get_user_stake_reward(user_id);
  if let Some(r) = reward{
       let stake_point =  if let Some(t) = r.stake_point_update_at{
            if t >0{
                r.stake_point
            }else {
                r.stake_point *POINT_DECIMAL
            }
            
        }else{
            r.stake_point *POINT_DECIMAL
        };

        let new_reward = StakeRewardRecord{
            stake_point:stake_point,
            ..r
        };
        Some(new_reward)
  }else{
    None
  }
}
