use candid::Encode;
use ic_cdk::api::management_canister::main::{CanisterId, CanisterInstallMode, WasmModule};
use wallet::domain::staking::InitStakingPoolArgument;

use crate::{error::Error, repositories};

use super::create_staking_pool;

pub(super) async fn serve(
    staking_pool_canister: CanisterId,
    staking_pool_wasm: WasmModule,
) -> Result<(), String> {
    let info = repositories::staking_pool::get(&staking_pool_canister)
        .ok_or_else(|| format!("Staking pool: {staking_pool_canister:?} not found"))?;

    let arg: InitStakingPoolArgument = info.into();

    let arg_bytes =
        Encode!(&arg).map_err(|e| Error::CandidEncodeError(e.to_string()).to_string())?;

    create_staking_pool::install_staking_pool_canister_code(
        staking_pool_canister,
        staking_pool_wasm,
        CanisterInstallMode::Upgrade(None),
        arg_bytes,
    )
    .await
}
