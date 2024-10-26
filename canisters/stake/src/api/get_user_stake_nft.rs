use candid::Principal;

use crate::{domain::StakeNFT, repositories::stake_nft};

pub fn serve(user_id: Principal) -> Vec<StakeNFT> {
    let staked_nfts = stake_nft::get_all_stake_record();
    let mut user_nft = Vec::new();
    for s in staked_nfts {
        if s.from == user_id {
            user_nft.push(s);
        }
    }
    user_nft
}
