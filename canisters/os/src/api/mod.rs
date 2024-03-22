pub mod append_wallet_action;
pub mod count_wallet;
pub mod create_wallet_canister;
pub mod create_wallet_owner;
pub mod get_wallet_action;
pub mod list_wallet;

use candid::Principal;
use ic_cdk::export_candid;
use ic_ledger_types::{
    AccountBalanceArgs, AccountIdentifier, Tokens, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};

use crate::{
    constants::WALLET_WASM,
    domain::{Action, WalletAction, WalletOwner},
    error::Error,
};

/// Create a smart wallet canister, log the action, and store the wallet owner info
#[ic_cdk::update]
pub async fn create_wallet() -> Result<Principal, Error> {
    let os = ic_cdk::api::id();
    let owner = ic_cdk::caller();
    let created_at = ic_cdk::api::time();

    // create smart wallet canister
    let canister_id = create_wallet_canister::serve(vec![owner, os], WALLET_WASM.to_owned())
        .await
        .map_err(|msg| Error::CreateCanisterFailed { msg })?;

    append_wallet_action::serve(owner, Action::Create, created_at)?;

    create_wallet_owner::serve(owner, canister_id, created_at)?;

    Ok(canister_id)
}

#[ic_cdk::query]
pub fn count_wallet() -> u64 {
    count_wallet::serve()
}

#[ic_cdk::query]
pub fn list_wallet() -> Vec<WalletOwner> {
    list_wallet::serve()
}

#[ic_cdk::query]
pub fn get_wallet_action(idx: u64) -> Option<WalletAction> {
    get_wallet_action::serve(idx)
}

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

export_candid!();
