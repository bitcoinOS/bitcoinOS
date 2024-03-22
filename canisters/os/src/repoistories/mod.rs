use candid::Principal;

use crate::{
    domain::{Action, WalletAction, WalletOwner},
    error::Error,
};

pub mod wallet_action_stable;
pub mod wallet_owner_stable;

pub trait WalletOwnerRepository {
    fn insert_wallet_owner(
        &mut self,
        owner: Principal,
        canister_id: Principal,
        created_at: u64,
    ) -> Result<Option<WalletOwner>, Error>;

    fn count_wallet(&self) -> u64;

    fn list_wallet(&self) -> Vec<WalletOwner>;
}

pub trait WalletActionRepository {
    fn append_wallet_action(
        &self,
        operator: Principal,
        action: Action,
        op_time: u64,
    ) -> Result<u64, Error>;

    fn get(&self, idx: u64) -> Option<WalletAction>;
}
