use crate::{constants::POINT_DECIMAL, domain::StakeRewardRecord, repositories};

pub fn serve() -> Vec<StakeRewardRecord> {
    let user_stake_rewards = repositories::stake_reward::get_all_user_stake_reward();
    let mut new_user_rewards = Vec::new();
    for r in user_stake_rewards{
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
        new_user_rewards.push(new_reward);
    }
    new_user_rewards
}
