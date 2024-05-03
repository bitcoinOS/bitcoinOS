use ic_cdk::api::management_canister::main::CanisterId;

use crate::{context::STATE, domain::WalletOwner, error::Error};

pub(crate) fn count() -> u64 {
    STATE.with(|s| s.borrow().wallet_owners.len())
}

pub(crate) fn list_wallet() -> Vec<WalletOwner> {
    STATE.with(|s| s.borrow().wallet_owners.iter().map(|(_, w)| w).collect())
}

pub(crate) fn create_wallet_owner(
    owner: candid::Principal,
    canister_id: CanisterId,
    created_at: u64,
) -> Result<Option<WalletOwner>, Error> {
    STATE.with(|s| {
        let state = &mut s.borrow_mut();
        let wallets = &mut state.wallet_owners;

        if wallets.contains_key(&canister_id) {
            Err(Error::WalletAlreadyExists {
                wallet_id: canister_id.to_string(),
            })
        } else {
            let wallet_owner = WalletOwner {
                canister_id,
                owner,
                created_at,
            };

            wallets.insert(canister_id, wallet_owner.clone());
            Ok(Some(wallet_owner))
        }
    })
}
