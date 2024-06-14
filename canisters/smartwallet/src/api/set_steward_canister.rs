use ic_cdk::api::management_canister::main::CanisterId;

use crate::{error::WalletError, repositories};

pub(super) fn serve(canister_id: CanisterId) -> Result<String, WalletError> {
    repositories::metadata::set_steward_canister(canister_id)
}
