use candid::Principal;
use ic_cdk::api::is_controller;
use ic_cdk::export_candid;
use ic_cdk::{api::management_canister::main::CanisterId, init};

use crate::context::STATE;
use crate::domain::request::{InitArgument, StakeRequest, UnStakeRequest};
use crate::domain::{Metadata, StakeNFT};

mod add_controller;
mod get_all_stake_nft;
mod get_stake_nft;
mod get_user_stake_nft;
mod stake_nft;
mod unstake_nft;
use crate::error::Error;
use crate::repositories;

/// Add new controller to a canister
#[ic_cdk::update]
async fn add_controller(
    canister_id: CanisterId,
    controllers: Vec<Principal>,
) -> Result<bool, Error> {
    let caller = ic_cdk::caller();
    let os_canister = ic_cdk::id();

    let mut controllers = controllers;
    controllers.extend(vec![caller, os_canister]);

    if is_controller(&ic_cdk::caller()) {
        add_controller::serve(canister_id, controllers).await
    } else {
        Err(Error::UnAuthorized(ic_cdk::caller().to_string()))
    }
}
#[ic_cdk::update]
async fn stake_nft(stake_request: StakeRequest) -> Result<bool, Error> {
    stake_nft::serve(stake_request).await
}
#[ic_cdk::update]
async fn unstake_nft(unstake_request: UnStakeRequest) -> Result<bool, Error> {
    unstake_nft::serve(unstake_request).await
}
/// --------------------- Queries interface of this canister -------------------

/// Returns metadata of os canister
#[ic_cdk::query]
fn metadata() -> Metadata {
    get_metadata()
}
#[ic_cdk::query]
fn get_stake_nft(nft_canister: Principal, nft_id: u32) -> Option<StakeNFT> {
    get_stake_nft::serve(nft_canister, nft_id)
}

#[ic_cdk::query]
fn get_user_stake_nft(user_id: Principal) -> Vec<StakeNFT> {
    get_user_stake_nft::serve(user_id)
}
#[ic_cdk::query]
fn get_all_stake_nft() -> Vec<StakeNFT> {
    get_all_stake_nft::serve()
}

fn get_metadata() -> Metadata {
    repositories::metadata::get_metadata()
}

#[init]
fn init(args: InitArgument) {
    STATE.with(|s| {
        let state = &mut s.borrow_mut();
        state
            .metadata
            .set(Metadata {
                user_canister: args.user_canister,
                os_canister: args.os_canister,
            })
            .expect("Failed to init metadata of os canister");
    });
}

export_candid!();
