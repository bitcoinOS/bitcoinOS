use candid::Encode;
use ic_cdk::api::management_canister::main::{CanisterId, CanisterInstallMode, WasmModule};

use crate::{domain::request::InitStakingPoolArgument, error::Error, repositories};

use super::create_staking_pool;

pub(super) async fn serve(
    staking_pool_canister: CanisterId,
    staking_pool_wasm: WasmModule,
) -> Result<(), String> {
    let info = repositories::staking_pool::get(&staking_pool_canister)
        .ok_or_else(|| format!("Wallet: {staking_pool_canister:?} not found"))?;

    let arg = InitStakingPoolArgument {
        name: info.name,
        network: info.network,
        description: info.description,
        annual_interest_rate: info.annual_interest_rate,
        duration_in_day: info.duration_in_day,
        os_canister: info.os_canister,
        steward_canister: info.steward_canister.unwrap_or(CanisterId::anonymous()),
    };

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
