use ic_cdk::api::management_canister::main::CanisterId;

use crate::repositories;

pub(super) fn serve(canister_id: CanisterId) -> String {
    let resp = repositories::metadata::set_steward_canister(canister_id);

    if resp.is_ok() {
        canister_id.to_string()
    } else {
        "".to_string()
    }
}

