use ic_cdk::api::management_canister::main::CanisterId;

use crate::{domain::WalletInfo, error::Error, repositories};

pub(super) fn serve(
    wallet_canister: CanisterId,
    bitcoin_address: String,
) -> Result<WalletInfo, Error> {
    repositories::wallet_info::update_bitcoin_address(wallet_canister, bitcoin_address)
}
