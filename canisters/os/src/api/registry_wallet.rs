use candid::Principal;
use ic_cdk::api::management_canister::main::CanisterId;

use crate::{
    domain::{WalletInfo, WalletInfoKey},
    error::Error,
    repositories,
};

pub fn serve(wallet_info: WalletInfo) -> Result<(), Error> {
    // repositories::wallet_owner::create_wallet_owner(owner, canister_id, wallet_info.created_at)?;
    repositories::wallet_info::save(wallet_info)
}
