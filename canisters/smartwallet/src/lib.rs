pub mod api;
pub mod bitcoin;
pub mod constants;
pub mod context;
pub mod domain;
pub mod error;
pub mod rgb;

use crate::context::STATE;
use crate::domain::{
    request::{TransferRequest, UpdateKeyRequest},
    response::NetworkResponse,
    Metadata,
};
use crate::error::WalletError;

use base::tx::RawTransactionInfo;
use base::ICBitcoinNetwork;
use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::bitcoin::Satoshi;
use ic_cdk::export_candid;
use serde::Deserialize;

/// Create a wallet when init the wallet canister
#[ic_cdk::init]
async fn init(args: InitArgument) {
    ic_wasi_polyfill::init(&[0u8; 32], &[]);

    let network = args.network;
    let steward_canister = args.steward_canister;

    STATE.with(|m| {
        let mut state = m.borrow_mut();

        state
            .metadata
            .set(Metadata {
                network,
                steward_canister,
                key_name: args.key_name,
                ..Default::default()
            })
            .expect("Failed to init network");

        state.controllers.insert(ic_caller(), ic_time());
    });
}

#[ic_cdk::update]
fn issue_rgb20() -> String {
    rgb::issue_rgb20()
}

pub fn ic_caller() -> Principal {
    ic_cdk::caller()
}

pub fn ic_time() -> u64 {
    ic_cdk::api::time()
}

export_candid!();

#[derive(CandidType, Deserialize)]
struct InitArgument {
    network: ICBitcoinNetwork,
    steward_canister: Principal,
    key_name: String,
}
