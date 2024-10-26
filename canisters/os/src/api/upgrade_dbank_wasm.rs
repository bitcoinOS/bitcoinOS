use ic_cdk::api::management_canister::main::{CanisterId, CanisterInstallMode, WasmModule};

use crate::{domain::request::InitDBankArgument, repositories};

use super::create_dbank_canister;

pub(super) async fn serve(
    dbank_id: u64,
    dbank_canister: CanisterId,
    dbank_wasm: WasmModule,
) -> Result<(), String> {
    let info = repositories::dbank_info::get(&dbank_id)
        .ok_or_else(|| format!("DBank caniser: {dbank_canister:?} not found"))?;

    let arg: InitDBankArgument = info.into();

    create_dbank_canister::install_dbank_canister_code(
        dbank_canister,
        dbank_wasm,
        CanisterInstallMode::Upgrade(None),
        arg,
    )
    .await
}
