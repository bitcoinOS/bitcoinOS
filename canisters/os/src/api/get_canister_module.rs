use crate::domain::CanisterModuleInfo;
use crate::repositories::canister_module;
pub(super) async fn serve(canister_name: String) -> Option<CanisterModuleInfo> {
    canister_module::get_canister_module(canister_name)
}
