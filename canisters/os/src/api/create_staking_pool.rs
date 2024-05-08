use candid::{Encode, Principal};
use ic_cdk::api::management_canister::{
    bitcoin::BitcoinNetwork,
    main::{
        create_canister, install_code, CanisterId, CanisterInstallMode, CanisterSettings,
        CreateCanisterArgument, InstallCodeArgument, WasmModule,
    },
};

use crate::{
    constants::DEFAULT_CYCLES_PER_CANISTER, domain::request::InitStakingPoolArgument, error::Error,
};

pub(crate) async fn serve(
    name: String,
    description: String,
    annual_interest_rate: u64,
    duration_in_millisecond: u64,
    network: BitcoinNetwork,
    os_canister: CanisterId,
    staking_pool_wasm: WasmModule,
) -> Result<CanisterId, String> {
    // create wallet canister id
    let staking_canister_id = create_new_staking_pool_canister(vec![os_canister]).await?;

    ic_cdk::println!(
        "-------------- created staking pool canister id: {:?} --------------- \n",
        staking_canister_id.to_text()
    );

    let arg = InitStakingPoolArgument {
        name,
        description,
        annual_interest_rate,
        duration_in_millisecond,
        network,
        os_canister,
    };

    // Translate arg for CreateStaking
    let arg = Encode!(&arg).map_err(|e| Error::CandidEncodeError(e.to_string()).to_string())?;

    // install wallet wasm module
    install_staking_pool_canister_code(
        staking_canister_id,
        staking_pool_wasm,
        CanisterInstallMode::Install,
        arg,
    )
    .await?;

    Ok(staking_canister_id)
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

pub(super) async fn install_staking_pool_canister_code(
    staking_canister_id: CanisterId,
    staking_wasm: WasmModule,
    mode: CanisterInstallMode,
    arg: Vec<u8>,
) -> Result<(), String> {
    let install_args = InstallCodeArgument {
        mode,
        canister_id: staking_canister_id,
        wasm_module: staking_wasm,
        arg,
    };

    install_code(install_args)
        .await
        .map_err(|(code, msg)| format!("Created failed: code: {code:?}, msg: {msg:?}"))
}
