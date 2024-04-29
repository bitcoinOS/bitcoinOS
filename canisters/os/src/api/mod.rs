pub mod append_wallet_action;
pub mod count_staking_pool;
pub mod count_wallet;
pub mod create_staking_pool;
pub mod create_wallet;
pub mod get_wallet_action;
pub mod list_staking_pool;
pub mod list_wallet;
pub mod list_wallet_types;
pub mod registry_staking_pool;
pub mod registry_wallet;
pub mod staking_pool_increment_one;
pub mod wallet_counter_increment_one;

use candid::Principal;
use ic_cdk::{
    api::{is_controller, management_canister::main::CanisterId},
    export_candid, init,
};
use ic_ledger_types::{
    AccountBalanceArgs, AccountIdentifier, Tokens, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};

use crate::{
    constants::{STAKING_POOL_WASM, WALLET_WASM},
    context::STATE,
    domain::{
        request::{CreateStakingPoolRequest, InitArgument, InitStakingPoolArgument},
        Action, Metadata, StakingPoolInfo, WalletAction, WalletOwner,
    },
    error::Error,
    repositories::{self, staking_pool_counter},
};

/// ---------------- Update interface of this canister ------------------
///
/// Create a smart wallet canister, log the action, and store the wallet owner info
#[ic_cdk::update]
pub async fn create_wallet_canister(name: String) -> Result<Principal, Error> {
    let owner = ic_cdk::caller();
    let created_at = ic_cdk::api::time();

    let metadata = repositories::metadata::get_metadata();

    let canister_id = create_wallet::serve(name, owner, metadata, WALLET_WASM.to_owned())
        .await
        .map_err(|msg| Error::CreateCanisterFailed { msg })?;

    append_wallet_action::serve(owner, Action::Create, created_at)?;

    registry_wallet::serve(owner, canister_id, created_at)?;

    wallet_counter_increment_one::serve()?;

    Ok(canister_id)
}

/// Create a Staking Pool with given annualized interest rate and duration, name and description
#[ic_cdk::update]
async fn create_staking_pool_canister(arg: CreateStakingPoolRequest) -> Result<CanisterId, Error> {
    let owner = ic_cdk::caller();

    if is_controller(&owner) {
        return Err(Error::UnAuthorized(owner.to_string()));
    }

    let os_canister = ic_cdk::id();
    let created_at = ic_cdk::api::time();
    let metadata = repositories::metadata::get_metadata();
    let init_arg = InitStakingPoolArgument {
        name: arg.name,
        description: arg.description,
        annual_interest_rate: arg.annual_interest_rate,
        duration_in_month: arg.duration_in_month,
        network: metadata.network,
        os_canister,
    };

    let staking_pool_id =
        create_staking_pool::serve(init_arg.clone(), STAKING_POOL_WASM.to_owned())
            .await
            .map_err(|msg| Error::CreateCanisterFailed { msg })?;

    registry_staking_pool::serve(
        staking_pool_id,
        metadata.network,
        os_canister,
        created_at,
        init_arg,
    )?;

    staking_pool_increment_one::serve()?;

    Ok(staking_pool_id)
}

/// Returns the ICP balance of  this canister
#[ic_cdk::update]
async fn canister_balance() -> Tokens {
    match ic_ledger_types::account_balance(
        MAINNET_LEDGER_CANISTER_ID,
        AccountBalanceArgs {
            account: AccountIdentifier::new(&ic_cdk::api::id(), &DEFAULT_SUBACCOUNT),
        },
    )
    .await
    {
        Ok(t) => t,
        _ => Tokens::from_e8s(0),
    }
}

/// --------------------- Queries interface of this canister -------------------
///
/// Returns wallet counter
#[ic_cdk::query]
pub fn wallet_counter() -> u128 {
    repositories::wallet_counter::get_counter()
}

/// Returns the count of wallet created by os canister
#[ic_cdk::query]
pub fn count_wallet() -> u64 {
    count_wallet::serve()
}

/// Returns the list of wallet types
#[ic_cdk::query]
fn list_wallet_types() -> Vec<String> {
    list_wallet_types::serve()
}

/// Returns the list of wallets created by os canister
#[ic_cdk::query]
pub fn list_wallet() -> Vec<WalletOwner> {
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
pub fn get_wallet_action(idx: u64) -> Option<WalletAction> {
    get_wallet_action::serve(idx)
}

#[ic_cdk::query]
/// Returns metadata of os canister
pub fn metadata() -> Metadata {
    repositories::metadata::get_metadata()
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
            })
            .expect("Failed to init metadata of os canister");
    });
}

export_candid!();
