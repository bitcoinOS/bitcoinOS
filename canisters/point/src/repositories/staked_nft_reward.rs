use crate::{context::STATE, domain::NftRewardRecord};
use candid::Principal;

pub fn get_all_user_stake_nft_reward() -> Vec<NftRewardRecord> {
    STATE.with_borrow(|s| s.staked_nft_reward.iter().map(|(_, w)| w).collect())
}

pub fn get_user_stake_nft_reward(user_id: Principal) -> Option<NftRewardRecord> {
    STATE.with_borrow(|s| s.staked_nft_reward.get(&user_id))
}

pub fn save_reward(reward_record: NftRewardRecord) -> Option<NftRewardRecord> {
    STATE.with_borrow_mut(|s| {
        let records = &mut s.staked_nft_reward;
        records.insert(reward_record.user_id, reward_record)
    })
}
