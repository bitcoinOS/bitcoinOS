use candid::{Encode, Principal};
use ic_cdk::api::management_canister::main::{
    create_canister, install_code, CanisterId, CanisterInstallMode, CanisterSettings,
    CreateCanisterArgument, InstallCodeArgument, WasmModule,
};
use wallet::utils::ic_time;

use crate::{
    api::registry_dbank,
    constants::DEFAULT_CYCLES_PER_DBANK,
    domain::{request::InitDBankArgument, DBankInfo, Metadata},
    error::Error,
};

#[allow(clippy::too_many_arguments)]
pub async fn serve(
    dbank_id: u64,
    seq_in_os: u64,
    max_wallet_limit: u32,
    name: String,
    os: CanisterId,
    owner: Principal,
    metadata: Metadata,
    dbank_wasm: WasmModule,
) -> Result<CanisterId, String> {
    // create dbank canister id
    let dbank_canister = create_new_dbank_canister(
        vec![os],
        metadata.dbank_cycles.unwrap_or(DEFAULT_CYCLES_PER_DBANK),
    )
    .await?;

    ic_cdk::println!("created dbank canister id: {:?}", dbank_canister.to_text());

    let network = metadata.network;
    let steward_canister = metadata.steward_canister;

    let init_args = InitDBankArgument {
        dbank_id,
        seq_in_os,
        max_wallet_limit,
        name: name.clone(),
        network,
        steward_canister,
        owner: os,
    };

    // install dbank wasm module
    install_dbank_canister_code(
        dbank_canister,
        dbank_wasm,
        CanisterInstallMode::Install,
        init_args,
    )
    .await?;

    let info = DBankInfo {
        dbank_id,
        name,
        owner,
        dbank_canister,
        network,
        steward_canister,
        status: crate::domain::DBankStatus::Activing,
        start_seq_in_os: seq_in_os,
        current_seq_in_os: seq_in_os,
        max_wallet_limit,
        created_at: ic_time(),
    };

    // repositories::dbank_info::save(info).map_err(|e| e.to_string())?;
    registry_dbank::serve(info).map_err(|e| e.to_string())?;

    Ok(dbank_canister)
}

async fn create_new_dbank_canister(
    owners: Vec<Principal>,
    init_dbank_cycles: u64,
) -> Result<Principal, String> {
    let create_args = CreateCanisterArgument {
        settings: Some(CanisterSettings {
            controllers: Some(owners),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
            log_visibility: None,
            wasm_memory_limit: None,
        }),
    };

    create_canister(create_args, init_dbank_cycles as u128)
        .await
        .map_err(|(code, msg)| format!("Created failed: code: {code:?}, msg: {msg:?}"))
        .map(|(c,)| c.canister_id)
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn install_dbank_canister_code(
    dbank_canister: CanisterId,
    dbank_wasm: WasmModule,
    mode: CanisterInstallMode,
    init_args: InitDBankArgument,
) -> Result<(), String> {
    let arg =
        Encode!(&init_args).map_err(|e| Error::CandidEncodeError(e.to_string()).to_string())?;

    let install_args = InstallCodeArgument {
        mode,
        canister_id: dbank_canister,
        wasm_module: dbank_wasm,
        arg,
    };

    install_code(install_args)
        .await
        .map_err(|(code, msg)| format!("Created failed: code: {code:?}, msg: {msg:?}"))
}
