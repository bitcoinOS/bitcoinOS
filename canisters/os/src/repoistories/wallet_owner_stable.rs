use std::cell::RefCell;

use crate::{domain::WalletOwner, error::Error, WalletOwnerStable};

use super::WalletOwnerRepository;

pub struct WalletOwnerStableRepository<'a> {
    pub owners: &'a RefCell<WalletOwnerStable>,
}

impl<'a> From<&'a RefCell<WalletOwnerStable>> for WalletOwnerStableRepository<'a> {
    fn from(owners: &'a RefCell<WalletOwnerStable>) -> Self {
        Self { owners }
    }
}

impl<'a> WalletOwnerRepository for WalletOwnerStableRepository<'a> {
    fn insert_wallet_owner(
        &mut self,
        owner: candid::Principal,
        canister_id: candid::Principal,
        created_at: u64,
    ) -> Result<Option<WalletOwner>, Error> {
        if self.owners.borrow().contains_key(&canister_id) {
            Err(Error::AlreadyExists)
        } else {
            let wallet_owner = WalletOwner {
                canister_id,
                owner,
                created_at,
            };

            Ok(self.owners.borrow_mut().insert(canister_id, wallet_owner))
        }
    }

    fn count_wallet(&self) -> u64 {
        self.owners.borrow().len()
    }

    fn list_wallet(&self) -> Vec<WalletOwner> {
        self.owners.borrow().iter().map(|(_, w)| w).collect()
    }
}
