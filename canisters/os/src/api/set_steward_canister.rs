use ic_cdk::api::management_canister::main::CanisterId;

use crate::{error::Error, repositories};

pub(super) fn serve(canister_id: CanisterId) -> Result<String, Error> {
    repositories::metadata::set_steward_canister(canister_id)
}
