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
    STATE.with(|s| s.borrow().wallet_infos.len())
}

pub(crate) fn list_wallet() -> Vec<WalletInfo> {
    STATE.with(|s| s.borrow().wallet_infos.iter().map(|(_, w)| w).collect())
}

pub(crate) fn find_info_by_owner_wallet(
    owner: Principal,
    wallet_canister: CanisterId,
) -> Option<WalletInfo> {
    STATE.with_borrow(|s| {
        s.wallet_infos.get(&WalletInfoKey {
            owner,
            wallet_canister,
        })
    })
}

pub(crate) fn save(info: WalletInfo) -> Result<(), Error> {
    STATE.with_borrow_mut(|s| {
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
    })
}

/// Find the wallet info list by owner
pub(crate) fn find_info_by_owner(owner: Principal) -> Vec<WalletInfo> {
    STATE.with_borrow(|s| {
        s.wallet_infos
            .range(range_owner_filter(owner))
            .map(|(_, info)| info)
            .collect()
    })
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
