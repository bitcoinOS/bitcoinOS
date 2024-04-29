pub mod append_wallet_action;
pub mod count_wallet;
pub mod counter_increment_one;
pub mod create_wallet_canister;
pub mod get_wallet_action;
pub mod list_wallet;
pub mod registry_wallet;

use candid::{Encode, Principal};
use ic_cdk::{export_candid, init};
use ic_ledger_types::{
    AccountBalanceArgs, AccountIdentifier, Tokens, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};

use crate::{
    constants::WALLET_WASM,
    context::STATE,
    domain::{
        request::{InitArgument, InitWalletArgument},
        Action, Metadata, WalletAction, WalletOwner,
    },
    error::Error,
    repositories,
};

/// ---------------- Update interface of this canister ------------------
///
/// Create a smart wallet canister, log the action, and store the wallet owner info
#[ic_cdk::update]
pub async fn create_wallet_canister(name: String) -> Result<Principal, Error> {
    let owner = ic_cdk::caller();
    let created_at = ic_cdk::api::time();

    let metadata = repositories::metadata::get_metadata();
    let network = metadata.network;

    let init_wallet = InitWalletArgument {
        name,
        network,
        steward_canister: metadata.steward_canister,
    };

    let init_arguemnt = Encode!(&init_wallet).unwrap();
    // create smart wallet canister
    let canister_id =
        create_wallet_canister::serve(vec![owner], WALLET_WASM.to_owned(), init_arguemnt)
            .await
            .map_err(|msg| Error::CreateCanisterFailed { msg })?;

    append_wallet_action::serve(owner, Action::Create, created_at)?;

    registry_wallet::serve(owner, canister_id, created_at)?;

    counter_increment_one::serve();

    Ok(canister_id)
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
/// Returns the count of wallet created by os canister
#[ic_cdk::query]
pub fn count_wallet() -> u64 {
    count_wallet::serve()
}

/// Returns the list of wallets created by os canister
#[ic_cdk::query]
pub fn list_wallet() -> Vec<WalletOwner> {
    list_wallet::serve()
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

#[ic_cdk::query]
pub fn counter() -> u128 {
    repositories::counter::get_counter()
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
