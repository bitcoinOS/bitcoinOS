use candid::Principal;
use wallet::utils::ic_time;

use crate::error::Error;
use crate::{
    context::STATE,
    domain::{StakeNFT, StakeNFTKey, StakeStatus},
};

pub(crate) fn get_stake_record(nft_canister: Principal, nft_id: u32) -> Option<StakeNFT> {
    STATE.with_borrow(|s| {
        let key = StakeNFTKey {
            nft_canister: nft_canister,
            nft_id: nft_id,
        };
        s.stake_nft.get(&key)
    })
}

pub(crate) fn get_all_stake_record() -> Vec<StakeNFT> {
    STATE.with_borrow(|s| s.stake_nft.iter().map(|(_, r)| r).collect())
}

pub(crate) fn unstake_nft(nft_canister: Principal, nft_id: u32) -> Result<bool, Error> {
    STATE.with_borrow_mut(|s| {
        let stake_nft_records = &mut s.stake_nft;
        let key = StakeNFTKey {
            nft_canister: nft_canister,
            nft_id: nft_id,
        };
        let nft = stake_nft_records.get(&key);
        if let Some(n) = nft {
            if n.stake_status == StakeStatus::Unstake {
                Ok(true)
            } else {
                let new_nft = StakeNFT {
                    stake_status: StakeStatus::Unstake,
                    unstake_at: ic_time(),
                    ..n
                };
                stake_nft_records.insert(key, new_nft);
                Ok(true)
            }
        } else {
            Err(Error::NFTUnStaked {
                nft_canister: nft_canister,
                nft_id: nft_id,
            })
        }
    })
}

pub(crate) fn stake_nft(stake_nft: StakeNFT) -> Result<bool, Error> {
    STATE.with_borrow_mut(|s| {
        let stake_nft_records = &mut s.stake_nft;
        let key = StakeNFTKey {
            nft_canister: stake_nft.nft_canister,
            nft_id: stake_nft.nft_id,
        };
        stake_nft_records.insert(key, stake_nft);
        Ok(true)
        // let nft = stake_nft_records.get(&key);
        // if let Some(n) = nft {
        //     if n.stake_status == StakeStatus::Satke {
        //         Err(Error::NFTHasStaked {
        //             nft_canister: stake_nft.nft_canister,
        //             nft_id: stake_nft.nft_id,
        //         })
        //     } else {
        //         stake_nft_records.insert(key, stake_nft);
        //         Ok(true)
        //     }
        // } else {
        //     stake_nft_records.insert(key, stake_nft);
        //     Ok(true)
        // }
    })
}
