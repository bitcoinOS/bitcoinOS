use ic_cdk::api::management_canister::main::CanisterId;

use crate::repositories;

pub(super) fn serve(steward_canister: CanisterId) -> String {
    let resp = repositories::metadata::set_steward_canister(steward_canister);

    if resp.is_ok() {
        steward_canister.to_string()
    } else {
        "".to_string()
    }
}
