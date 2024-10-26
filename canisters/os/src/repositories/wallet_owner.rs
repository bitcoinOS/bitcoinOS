// use ic_cdk::api::management_canister::main::CanisterId;

// use crate::{context::STATE, domain::WalletOwner, error::Error};

// pub(crate) fn count() -> u64 {
//     STATE.with(|s| s.borrow().wallet_owners.len())
// }

// pub(crate) fn get(wallet_canister: &CanisterId) -> Option<WalletOwner> {
//     STATE.with_borrow(|s| s.wallet_owners.get(wallet_canister))
// }

// pub(crate) fn save(
//     owner: candid::Principal,
//     canister_id: CanisterId,
//     created_at: u64,
// ) -> Result<Option<WalletOwner>, Error> {
//     STATE.with_borrow_mut(|s| {
//         let wallets = &mut s.wallet_owners;

//         if wallets.contains_key(&canister_id) {
//             Err(Error::WalletAlreadyExists {
//                 wallet_canister: canister_id.to_string(),
//             })
//         } else {
//             let wallet_owner = WalletOwner {
//                 canister_id,
//                 owner,
//                 created_at,
//             };

//             wallets.insert(canister_id, wallet_owner.clone());
//             Ok(Some(wallet_owner))
//         }
//     })
// }
