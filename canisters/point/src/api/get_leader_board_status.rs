use crate::{domain::LeaderBoardStatus, repositories};

pub fn serve() -> LeaderBoardStatus {
    repositories::leaderboard::get_leader_board_status()
}
