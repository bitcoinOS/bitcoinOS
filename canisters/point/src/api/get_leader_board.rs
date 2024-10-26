use crate::{domain::RewardRecord, repositories};

pub fn serve() -> Vec<RewardRecord> {
    repositories::leaderboard::get_leader_board()
}
