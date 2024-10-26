use crate::{domain::StakeRewardRecord, repositories};

pub fn serve() -> Vec<StakeRewardRecord> {
    repositories::leaderboard::get_stake_leader_board()
}
