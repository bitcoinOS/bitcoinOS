use ic_cdk::api::management_canister::main::{CanisterId, CanisterInstallMode, WasmModule};

use crate::repositories;

use super::create_wallet;

pub(super) async fn serve(
    wallet_canister: CanisterId,
    wallet_wasm: WasmModule,
) -> Result<(), String> {
    let walelt_owner = repositories::wallet_owner::get(&wallet_canister)
        .ok_or_else(|| format!("Wallet: {wallet_canister:?} not found"))?;

    let wallet_info =
        repositories::wallet_info::find_info_by_owner_wallet(walelt_owner.owner, wallet_canister)
            .ok_or_else(|| format!("Wallet: {wallet_canister:?} not found"))?;

    create_wallet::install_wallet_canister_code(
        wallet_canister,
        wallet_wasm,
        CanisterInstallMode::Upgrade,
        wallet_info.name,
        wallet_info.network,
        wallet_info.steward_canister,
        Some(wallet_info.owner),
    )
    .await
}
