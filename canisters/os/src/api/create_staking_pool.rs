use candid::{Encode, Principal};
use ic_cdk::api::management_canister::main::{
    create_canister, install_code, CanisterId, CanisterInstallMode, CanisterSettings,
    CreateCanisterArgument, InstallCodeArgument, WasmModule,
};

use crate::{constants::DEFAULT_CYCLES_PER_CANISTER, domain::request::InitStakingPoolArgument};

pub(crate) async fn serve(
    arg: InitStakingPoolArgument,
    staking_pool_wasm: WasmModule,
) -> Result<CanisterId, String> {
    // create wallet canister id
    let wallet_canister_id = create_new_staking_pool_canister(vec![arg.os_canister]).await?;

    ic_cdk::println!(
        "created wallet canister id: {:?}",
        wallet_canister_id.to_text()
    );

    // Translate arg for CreateStaking
    let arg = Encode!(&arg).unwrap();

    // install wallet wasm module
    install_staking_pool_canister_code(wallet_canister_id, staking_pool_wasm, arg).await?;

    Ok(wallet_canister_id)
}

async fn create_new_staking_pool_canister(owners: Vec<Principal>) -> Result<Principal, String> {
    let create_args = CreateCanisterArgument {
        settings: Some(CanisterSettings {
            controllers: Some(owners),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
        }),
    };

    create_canister(create_args, DEFAULT_CYCLES_PER_CANISTER)
        .await
        .map_err(|(code, msg)| format!("Created failed: code: {code:?}, msg: {msg:?}"))
        .map(|(c,)| c.canister_id)
}

async fn install_staking_pool_canister_code(
    wallet_id: Principal,
    wallet_wasm: WasmModule,
    arg: Vec<u8>,
) -> Result<(), String> {
    let install_args = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id: wallet_id,
        wasm_module: wallet_wasm,
        arg,
    };

    install_code(install_args)
        .await
        .map_err(|(code, msg)| format!("Created failed: code: {code:?}, msg: {msg:?}"))
}
