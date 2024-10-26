pub mod api;
pub mod constants;
pub mod context;
pub mod domain;
pub mod error;
pub mod repositories;

use crate::domain::{request::RedeemRequest, response::NetworkResponse, Metadata, RedeemLog};
use wallet::error::StakingError;

use ic_cdk::api::management_canister::bitcoin::Satoshi;
use ic_cdk::api::management_canister::main::CanisterId;
use ic_cdk::export_candid;
use wallet::domain::request::{
    RegisterStakingRecordRequest, TransferRequest, UpdateStakingPoolInfoRequest,
    UpdateStakingPoolStatusRequest, UtxosRequest,
};
use wallet::domain::response::{UpdateStakingPoolInfoResponse, UtxosResponse};
use wallet::domain::staking::{InitStakingPoolArgument, StakingRecord};
use wallet::domain::{EcdsaKeyIds, TxId};
use wallet::utils::ic_time;

/// Create a wallet when init the wallet canister
#[ic_cdk::init]
async fn init(arg: InitStakingPoolArgument) {
    let owner = arg.os_canister;
    let name = arg.name;
    let network = arg.network;
    let os_canister = arg.os_canister;
    let ecdsa_key_id = EcdsaKeyIds::from(network).to_key_id();
    let updated_time = ic_time();
    // let point_canister = arg.point_canister;

    let metadata = Metadata {
        name,
        description: arg.description,
        network,
        annual_interest_rate: arg.annual_interest_rate,
        duration_in_day: arg.duration_in_day,
        os_canister,
        steward_canister: arg.steward_canister,
        ecdsa_key_id,
        updated_time,
        owner,
        status: arg.status.into(),
        start_time: arg.start_time,
        // stake_end_time: arg.stake_end_time,
        end_time: arg.end_time,
        fund_management: arg.fund_management.into(),
        minimum_stake_amount: arg.minimum_stake_amount,
        boost_rate: arg.boost_rate,
    };

    repositories::metadata::save(metadata).expect("Failed to init metadata");

    // TODO: schedule a task to check tx status is confirmed or not very hour, update staking record status when tx is confirmed for 6 blocks
    // let timer_id = ic_cdk_timers::set_timer_interval(ONE_HOURS, move || {
    //     ic_cdk::spawn(async move {
    //        sync_staking_record_from_pending_to_confirmed().await.expect("Failed to sync staking record task!")
    //     })
    // });
}

export_candid!();

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
