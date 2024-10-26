use std::borrow::Borrow;

use crate::{
    context::STATE,
    domain::{LeaderBoardStatus, RewardRecord, StakeRewardRecord},
};

pub fn get_leader_board() -> Vec<RewardRecord> {
    STATE.with_borrow(|s| s.user_rank.iter().map(|(_, w)| w).collect())
}

pub fn get_leader_board_status() -> LeaderBoardStatus {
    STATE.with_borrow(|s| s.borrow().leader_board_status.get().clone())
}

pub fn update_leader_board(seq: u64, reward_record: RewardRecord) {
    STATE.with_borrow_mut(|s| {
        let user_rank = &mut s.user_rank;
        user_rank.insert(seq, reward_record);
    });
}

pub fn remove_leader_board(seq: u64) {
    STATE.with_borrow_mut(|s| {
        let user_rank = &mut s.user_rank;
        user_rank.remove(&seq);
    });
}

pub fn update_leader_board_status(leader_board_status: LeaderBoardStatus) {
    STATE
        .with_borrow_mut(|s| s.leader_board_status.set(leader_board_status))
        .expect("update leader board status fail");
}

pub fn update_stake_leader_board(seq: u64, reward_record: StakeRewardRecord) {
    STATE.with_borrow_mut(|s| {
        let user_rank = &mut s.user_stake_rank;
        user_rank.insert(seq, reward_record);
    });
}

pub fn remove_stake_leader_board(seq: u64) {
    STATE.with_borrow_mut(|s| {
        let user_stake_rank = &mut s.user_stake_rank;
        user_stake_rank.remove(&seq);
    });
}

pub fn get_stake_leader_board() -> Vec<StakeRewardRecord> {
    STATE.with_borrow(|s| s.user_stake_rank.iter().map(|(_, w)| w).collect())
}
