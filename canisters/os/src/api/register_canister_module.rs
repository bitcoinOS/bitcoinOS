use crate::domain::CanisterModuleInfo;
use crate::error::Error;
use crate::repositories::canister_module;
use ic_cdk::api::management_canister::main::CanisterId;

pub(super) async fn serve(canister_name: String, canister_id: CanisterId) -> Result<bool, Error> {
    let created_at = ic_cdk::api::time();
    let canister_module_info = CanisterModuleInfo {
        canister_name,
        canister_id,
        created_at,
    };

    canister_module::add_canister_module(canister_module_info)
}
