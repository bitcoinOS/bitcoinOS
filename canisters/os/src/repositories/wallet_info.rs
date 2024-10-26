use std::ops::RangeBounds;

use candid::Principal;
use ic_cdk::api::management_canister::main::CanisterId;

use crate::{
    constants::{PRINCIPAL_MAX, PRINCIPAL_MIN},
    context::STATE,
    domain::{WalletInfo, WalletInfoKey},
    error::Error,
};

pub(crate) fn count() -> u64 {
    STATE.with_borrow(|s| s.wallet_infos.len() + s.dbank_wallet_infos.len())
}

pub(crate) fn list_wallet() -> Vec<WalletInfo> {
    let mut wallets: Vec<_> = STATE.with_borrow(|s| {
        let mut wallets: Vec<_> = s.wallet_infos.iter().map(|(_, w)| w).collect();
        let mut wallets2: Vec<_> = s.dbank_wallet_infos.iter().map(|(_, w)| w).collect();
        wallets.append(&mut wallets2);
        wallets
    });

    wallets.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    wallets
}

// pub(crate) fn find_info_by_owner_wallet(
//     owner: Principal,
//     wallet_canister: CanisterId,
// ) -> Option<WalletInfo> {
//     STATE.with_borrow(|s| {
//         s.wallet_infos.get(&WalletInfoKey {
//             owner,
//             wallet_canister,
//         })
//     })
// }

/// When wallet is a shared wallet in dbank canister, save it to dbank wallet info
pub(crate) fn save(info: WalletInfo) -> Result<(), Error> {
    STATE.with_borrow_mut(|s| {
        if info.is_share_wallet() {
            if s.dbank_wallet_infos.contains_key(&info.owner) {
                Err(Error::WalletAlreadyExists {
                    wallet_canister: info.wallet_canister.to_string(),
                })
            } else {
                s.dbank_wallet_infos.insert(info.owner, info);
                Ok(())
            }
        } else {
            let wallets = &mut s.wallet_infos;

            let key = WalletInfoKey {
                owner: info.owner,
                wallet_canister: info.wallet_canister,
            };

            if wallets.contains_key(&key) {
                Err(Error::WalletAlreadyExists {
                    wallet_canister: info.wallet_canister.to_string(),
                })
            } else {
                wallets.insert(key, info);

                Ok(())
            }
        }
    })
}

pub(crate) fn update_bitcoin_address(
    canister_id: CanisterId,
    bitcoin_address: String,
) -> Result<WalletInfo, Error> {
    STATE.with_borrow_mut(|state| {
        let owner = state.wallet_owners.get(&canister_id);
        if owner.is_none() {
            return Err(Error::WalletNotFound(canister_id.to_string()));
        }

        let wallets = &mut state.wallet_infos;

        let key = WalletInfoKey {
            owner: owner.unwrap().owner,
            wallet_canister: canister_id,
        };
        match wallets.get(&key) {
            Some(info) => {
                let new_info = WalletInfo {
                    bitcoin_address,
                    ..info
                };
                wallets.insert(key, new_info.clone());

                Ok(new_info)
            }
            None => Err(Error::UnAuthorized(canister_id.to_string())),
        }
    })
}

/// Find the wallet info list by owner
pub(crate) fn find_info_by_owner(owner: Principal) -> Vec<WalletInfo> {
    STATE.with_borrow(|s| match s.dbank_wallet_infos.get(&owner) {
        Some(w) => vec![w],
        None => s
            .wallet_infos
            .range(range_owner_filter(owner))
            .map(|(_, info)| info)
            .collect(),
    })
}

/// Count the wallet info list by owner
pub(crate) fn count_wallet_by_owner(owner: Principal) -> usize {
    find_info_by_owner(owner).len()
}

fn range_owner_filter(owner: Principal) -> impl RangeBounds<WalletInfoKey> {
    let start = WalletInfoKey {
        owner,
        wallet_canister: PRINCIPAL_MIN,
    };

    let end = WalletInfoKey {
        owner,
        wallet_canister: PRINCIPAL_MAX,
    };

    start..end
}
