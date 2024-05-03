use candid::Principal;
use ic_cdk::api::management_canister::main::CanisterId;

use crate::{context::STATE, domain::WalletInfo, error::Error};

pub(crate) fn count() -> u64 {
    STATE.with(|s| s.borrow().wallet_owners.len())
}

pub(crate) fn list_wallet() -> Vec<WalletInfo> {
    STATE.with(|s| s.borrow().wallet_infos.iter().map(|(_, w)| w).collect())
}

pub(crate) fn save(
    owner: candid::Principal,
    canister_id: CanisterId,
    info: WalletInfo,
) -> Result<(), Error> {
    STATE.with(|s| {
        let state = &mut s.borrow_mut();
        let wallets = &mut state.wallet_infos;

        if wallets.contains_key(&(owner, canister_id)) {
            Err(Error::WalletAlreadyExists {
                wallet_id: canister_id.to_string(),
            })
        } else {
            wallets.insert((owner, canister_id), info);
            Ok(())
        }
    })
}

/// Find the wallet info list by owner
/// TODO: FIX with range query
pub(crate) fn find_wallet_info_by_owner(owner: Principal) -> Vec<WalletInfo> {
    STATE.with_borrow(|s| {
        s.wallet_infos
            .iter()
            .filter(|((o, _), _)| o == &owner)
            .map(|(_, info)| info)
            .collect()
    })
}

// fn principal_canister_range(
//     owner: Principal,
//     start: CanisterId,
// ) -> Vec<WalletInfo> {
//     STATE.with_borrow(|s|{
//         s.wallet_infos.range((owner, start))
//         .map(|(_, info)| info)
//         .collect()
//     })
// }
