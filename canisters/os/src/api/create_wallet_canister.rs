use candid::Principal;
use ic_cdk::api::management_canister::main::{
    create_canister, install_code, CanisterInstallMode, CanisterSettings, CreateCanisterArgument,
    InstallCodeArgument, WasmModule,
};

use crate::constants::CYCLES_PER_WALLET_CANISTER;

pub async fn serve(owners: Vec<Principal>, wallet_wasm: WasmModule) -> Result<Principal, String> {
    // create wallet canister id
    let wallet_canister_id = create_new_wallet_canister(owners).await?;

    ic_cdk::println!(
        "created wallet canister id: {:?}",
        wallet_canister_id.to_text()
    );

    // install wallet wasm module
    install_wallet_canister_code(wallet_canister_id, wallet_wasm, vec![]).await?;

    Ok(wallet_canister_id)
}

async fn create_new_wallet_canister(owners: Vec<Principal>) -> Result<Principal, String> {
    let create_args = CreateCanisterArgument {
        settings: Some(CanisterSettings {
            controllers: Some(owners),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
        }),
    };

    create_canister(create_args, CYCLES_PER_WALLET_CANISTER)
        .await
        .map_err(|(code, msg)| format!("Created failed: code: {code:?}, msg: {msg:?}"))
        .map(|(c,)| c.canister_id)
}

async fn install_wallet_canister_code(
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
