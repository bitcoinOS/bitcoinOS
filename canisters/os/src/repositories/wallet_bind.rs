use std::ops::RangeBounds;

use crate::{
    // constants::MAX_BIND_WALLET,
    context::STATE,
    domain::{BindWalletStatus, UserBindKey, WalletBindInfo},
    error::Error,
};
use candid::Principal;
use wallet::utils::ic_time;

/// Find the wallet info list by owner
pub(crate) fn get_user_bind_wallets(owner: Principal) -> Option<Vec<WalletBindInfo>> {
    STATE.with_borrow(|s| {
        let user_bind_count = s.user_bind_wallet_count.get(&owner);
        if let Some(c) = user_bind_count {
            let bind_info = s
                .user_bind_wallet_info
                .range(range_owner_filter(owner, c))
                .map(|(_, info)| info)
                .filter(|v| v.bind_status == BindWalletStatus::Binded)
                .collect();
            Some(bind_info)
        } else {
            None
        }
    })
}

// /// Count the wallet info list by owner
// pub(crate) fn count_wallet_by_owner(owner: Principal) -> usize {
//     find_info_by_owner(owner).len()
// }

fn range_owner_filter(owner: Principal, index: u64) -> impl RangeBounds<UserBindKey> {
    let start = UserBindKey {
        user_id: owner,
        bind_id: 0,
    };

    let end = UserBindKey {
        user_id: owner,
        bind_id: index,
    };

    start..=end
}

pub fn add_bind_wallet(wallet_info: WalletBindInfo) -> Result<bool, Error> {
    STATE.with_borrow_mut(|s| {
        let wallet_map_bind = &mut s.wallet_map_bind;
        let wallet_address = wallet_info.clone().wallet_address;
        if wallet_map_bind.contains_key(&wallet_address) {
            Err(Error::BindWalletError(wallet_address.clone()))
        } else {
            let wallet_list = &mut s.user_bind_wallet_info;
            let bind_count_stable = &mut s.user_bind_wallet_count;
            let user_bind_count = bind_count_stable.get(&wallet_info.user_id);
            let new_user_bind_key = match user_bind_count {
                Some(bc) => UserBindKey {
                    user_id: wallet_info.user_id,
                    bind_id: bc + 1,
                },
                None => UserBindKey {
                    user_id: wallet_info.user_id,
                    bind_id: 0,
                },
            };
            wallet_map_bind.insert(wallet_address.clone(), new_user_bind_key);
            let user_bind_key = UserBindKey {
                user_id: wallet_info.user_id,
                bind_id: new_user_bind_key.bind_id,
            };
            wallet_list.insert(user_bind_key, wallet_info.clone());
            bind_count_stable.insert(wallet_info.user_id, new_user_bind_key.bind_id);
            Ok(true)
        }
    })
}

pub fn remove_bind_wallet(wallet_info: WalletBindInfo) -> Result<bool, Error> {
    STATE.with_borrow_mut(|s| {
        let wallet_map_bind = &mut s.wallet_map_bind;
        if wallet_map_bind.contains_key(&wallet_info.wallet_address) {
            let user_bind_key = wallet_map_bind.get(&wallet_info.wallet_address).unwrap();
            let wallet_list = &mut s.user_bind_wallet_info;
            wallet_map_bind.remove(&wallet_info.wallet_address);
            let wallet_bind_info = wallet_list.get(&user_bind_key);
            if let Some(w) = wallet_bind_info {
                let new_bind_info = WalletBindInfo {
                    bind_status: BindWalletStatus::Unbind,
                    unbind_time: ic_time(),
                    ..w
                };
                wallet_list.insert(user_bind_key, new_bind_info);
            }
            // wallet_list.remove(&user_bind_key);
            Ok(true)
        } else {
            Err(Error::UnBindWallet(wallet_info.wallet_address))
        }
    })
}

pub fn check_wallet_bind_user(user_id: Principal, wallet_address: String) -> bool {
    STATE.with_borrow(|s| {
        let wallet_list = s.wallet_map_bind.get(&wallet_address);
        if let Some(w) = wallet_list {
            if w.user_id == user_id {
                true
            } else {
                false
            }
        } else {
            false
        }
    })
}
