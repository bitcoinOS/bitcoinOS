use wallet::domain::reward::{Reward, RewardType};

use crate::repositories;
pub async fn serve(r: Reward) {
    match r.reward_type {
        RewardType::Login => {
            let _ = repositories::box_record::add_login_box(r.user_id);
            ic_cdk::print("point add reward 1");
        }
        RewardType::Invite => {
            ic_cdk::print("point add reward 2");
            let _ = repositories::invite_reward::add_invite(r.user_id, r.invite_user.unwrap());
        }
        _ => {}
    }
}
