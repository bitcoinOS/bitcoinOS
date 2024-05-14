mod append_wallet_action;
mod confirm_staking_record;
mod count_staking_pool;
mod count_wallet;
mod create_staking_pool;
mod create_wallet;
mod get_wallet_action;
mod list_staking_pool;
mod list_wallet;
mod list_wallet_types;
mod my_wallet;
mod redeemed_staking_record;
mod registry_staking_pool;
mod registry_wallet;
mod set_wallet_cycles;
mod staking_pool_increment_one;
mod upgrade_staking_pool_wasm;
mod upgrade_wallet_wasm;
mod wallet_counter_increment_one;

use candid::Principal;
use ic_cdk::{
    api::{is_controller, management_canister::main::CanisterId},
    export_candid, init,
};

use crate::{
    constants::{DEFAULT_CYCLES_PER_CANISTER, STAKING_POOL_WASM, WALLET_WASM},
    context::STATE,
    domain::{
        request::{CreateStakingPoolRequest, InitArgument},
        Action, Metadata, StakingPoolInfo, WalletAction, WalletInfo,
    },
    error::Error,
    repositories::{self, staking_pool_counter},
};

/// ---------------- Update interface of this canister ------------------
///
/// Create a smart wallet canister, log the action, and store the wallet owner info
#[ic_cdk::update]
async fn create_wallet_canister(name: String) -> Result<Principal, Error> {
    let os = ic_cdk::id();
    let owner = ic_cdk::caller();
    let created_at = ic_cdk::api::time();

    let metadata = get_metadata();

    let wallet_canister = create_wallet::serve(
        name.clone(),
        os,
        owner,
        metadata.clone(),
        WALLET_WASM.to_owned(),
    )
    .await
    .map_err(|msg| Error::CreateCanisterFailed { msg })?;

    append_wallet_action::serve(owner, Action::Create, created_at)?;

    let wallet_address = fetch_wallet_address(wallet_canister).await?;

    let wallet_info = WalletInfo {
        name,
        owner,
        wallet_canister,
        bitcoin_address: wallet_address,
        network: metadata.network,
        steward_canister: metadata.steward_canister,
        created_at,
    };

    registry_wallet::serve(wallet_info)?;

    wallet_counter_increment_one::serve()?;

    Ok(wallet_canister)
}

/// Update wallet with new wasm file for tests
/// TODO: Remove this once tests when deploy to mainnet
#[ic_cdk::update]
async fn upgrade_wallet_wasm(wallet_canister: CanisterId) -> Result<(), String> {
    if is_controller(&ic_cdk::caller()) {
        upgrade_wallet_wasm::serve(wallet_canister, WALLET_WASM.to_owned()).await
    } else {
        Err("UnAuthorized".to_string())
    }
}

/// Create a Staking Pool with given annualized interest rate and duration, name and description
#[ic_cdk::update]
async fn create_staking_pool_canister(
    arg: CreateStakingPoolRequest,
) -> Result<StakingPoolInfo, Error> {
    let owner = ic_cdk::caller();

    if !is_controller(&owner) {
        return Err(Error::UnAuthorized(owner.to_string()));
    }

    let os_canister = ic_cdk::id();
    let created_at = ic_cdk::api::time();
    let metadata = get_metadata();

    let staking_pool_id = create_staking_pool::serve(
        arg.name.clone(),
        arg.description.clone(),
        arg.annual_interest_rate,
        arg.duration_in_day,
        metadata.network,
        os_canister,
        STAKING_POOL_WASM.to_owned(),
        metadata.wallet_cycles,
    )
    .await
    .map_err(|msg| Error::CreateCanisterFailed { msg })?;

    ic_cdk::print("Created staking pool canister ----------- \n");

    let staking_pool_address = fetch_wallet_address(staking_pool_id).await?;

    let info = registry_staking_pool::serve(
        staking_pool_id,
        metadata.network,
        os_canister,
        created_at,
        arg.name,
        arg.description,
        arg.annual_interest_rate,
        arg.duration_in_day,
        staking_pool_address,
    )?;

    staking_pool_increment_one::serve()?;

    Ok(info)
}

/// Update staking pool with new wasm file for tests
/// TODO: Remove this once tests when deploy to mainnet
#[ic_cdk::update]
async fn upgrade_staking_pool_wasm(staking_pool_canister: CanisterId) -> Result<(), String> {
    if is_controller(&ic_cdk::caller()) {
        upgrade_staking_pool_wasm::serve(staking_pool_canister, STAKING_POOL_WASM.to_owned()).await
    } else {
        Err("UnAuthorized".to_string())
    }
}

/// Update the default cycles for a wallet canister when creating a wallet
/// NOTE: Only controller can update
#[ic_cdk::update]
fn set_wallet_cycles(wallet_cycles: u64) -> Result<u64, Error> {
    if is_controller(&ic_cdk::caller()) {
        set_wallet_cycles::serve(wallet_cycles)
    } else {
        Err(Error::UnAuthorized(ic_cdk::caller().to_string()))
    }
}

/// Sync the staking record confirmed or not
#[ic_cdk::update]
async fn confirm_staking_record(staking_canister: CanisterId) -> Result<bool, Error> {
    let caller = ic_cdk::caller();

    if !is_controller(&caller) {
        return Err(Error::UnAuthorized(caller.to_string()));
    }

    confirm_staking_record::serve(staking_canister).await
}

/// Sync the staking record confirmed or not
#[ic_cdk::update]
async fn redeemed_staking_record(staking_canister: CanisterId) -> Result<bool, Error> {
    let caller = ic_cdk::caller();

    if !is_controller(&caller) {
        return Err(Error::UnAuthorized(caller.to_string()));
    }

    redeemed_staking_record::serve(staking_canister).await
}

/// --------------------- Queries interface of this canister -------------------
///
/// Returns wallet counter, it will always increment by one
#[ic_cdk::query]
fn wallet_counter() -> u128 {
    repositories::wallet_counter::get_counter()
}

/// Returns the count of wallet created by os canister
#[ic_cdk::query]
fn count_wallet() -> u64 {
    count_wallet::serve()
}

/// Returns the wallet info list of the caller
#[ic_cdk::query]
fn my_wallets() -> Vec<WalletInfo> {
    let sender = ic_cdk::caller();
    my_wallet::serve(sender)
}

/// Returns the list of wallet types
#[ic_cdk::query]
fn list_wallet_type() -> Vec<String> {
    list_wallet_types::serve()
}

/// Returns the list of wallets created by os canister
#[ic_cdk::query]
fn list_wallet() -> Vec<WalletInfo> {
    list_wallet::serve()
}

/// Returns staking pool counter
#[ic_cdk::query]
fn count_staking_pool() -> u128 {
    staking_pool_counter::get_counter()
}

/// Returns the list of staking pools
#[ic_cdk::query]
fn list_staking_pool() -> Vec<StakingPoolInfo> {
    list_staking_pool::serve()
}

/// Returns the create wallet action for given index
#[ic_cdk::query]
fn get_wallet_action(idx: u64) -> Option<WalletAction> {
    get_wallet_action::serve(idx)
}

/// Returns metadata of os canister
#[ic_cdk::query]
fn metadata() -> Metadata {
    get_metadata()
}

/// Returns the timestamp of this canister
#[ic_cdk::query]
fn timestamp() -> u64 {
    ic_cdk::api::time()
}

#[init]
fn init(args: InitArgument) {
    STATE.with(|s| {
        let state = &mut s.borrow_mut();
        state
            .metadata
            .set(Metadata {
                network: args.network,
                steward_canister: args.steward_canister,
                wallet_cycles: args.wallet_cycles.unwrap_or(DEFAULT_CYCLES_PER_CANISTER),
            })
            .expect("Failed to init metadata of os canister");
    });
}

export_candid!();

async fn fetch_wallet_address(staking_pool_canister: CanisterId) -> Result<String, Error> {
    let resp: Result<(String,), _> = ic_cdk::call(staking_pool_canister, "p2pkh_address", ((),))
        .await
        .map_err(|msg| Error::GetStakingPoolAddressFailed {
            msg: format!("{msg:?}"),
        });

    resp.map(|(address,)| address)
}

fn get_metadata() -> Metadata {
    repositories::metadata::get_metadata()
}
