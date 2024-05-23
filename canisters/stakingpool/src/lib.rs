pub mod api;
pub mod constants;
pub mod context;
pub mod domain;
pub mod error;
pub mod repositories;

use crate::context::STATE;
use crate::domain::{
    request::{RedeemRequest, RegisterStakingRequest},
    response::NetworkResponse,
    Metadata, RedeemLog,
};
use crate::error::StakingError;

use candid::{CandidType, Principal};

use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, Satoshi};
use ic_cdk::export_candid;
use serde::Deserialize;
use wallet::domain::request::UtxosRequest;
use wallet::domain::response::UtxosResponse;
use wallet::domain::staking::StakingRecord;
use wallet::domain::{EcdsaKeyIds, TxId};
use wallet::utils::ic_time;

/// Create a wallet when init the wallet canister
#[ic_cdk::init]
async fn init(arg: InitArgument) {
    ic_wasi_polyfill::init(&[0u8; 32], &[]);

    let owner = arg.os_canister;
    let name = arg.name;
    let network = arg.network;
    let os_canister = arg.os_canister;
    let ecdsa_key_id = EcdsaKeyIds::from(network).to_key_id();
    let updated_time = ic_time();

    STATE.with(|s| {
        let metadata = &mut s.borrow_mut().metadata;
        metadata
            .set(Metadata {
                name,
                description: arg.description,
                network,
                annual_interest_rate: arg.annual_interest_rate,
                duration_in_day: arg.duration_in_day,
                os_canister,
                ecdsa_key_id,
                updated_time,
                owner,
            })
            .expect("Failed to init metadata")
    });

    // TODO: schedule a task to check tx status is confirmed or not very hour, update staking record status when tx is confirmed for 6 blocks
    // let timer_id = ic_cdk_timers::set_timer_interval(ONE_HOURS, move || {
    //     ic_cdk::spawn(async move {
    //        sync_staking_record_from_pending_to_confirmed().await.expect("Failed to sync staking record task!")
    //     })
    // });
}

// #[ic_cdk::post_upgrade]
// async fn post_upgrade() {

// }

export_candid!();

#[derive(CandidType, Deserialize)]
struct InitArgument {
    name: String,
    description: String,
    network: BitcoinNetwork,
    // the annual interest rate of the staking pool will less than 10000, it will divide by 10000 for compute
    annual_interest_rate: u16,
    duration_in_day: u64,
    os_canister: Principal,
}

// In the following, we register a custom getrandom implementation because
// otherwise getrandom (which is a dependency of k256) fails to compile.
// This is necessary because getrandom by default fails to compile for the
// wasm32-unknown-unknown target (which is required for deploying a canister).
// Our custom implementation always fails, which is sufficient here because
// we only use the k256 crate for verifying secp256k1 signatures, and such
// signature verification does not require any randomness.
getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}
