pub mod api;
pub mod bitcoin;
pub mod constants;
pub mod context;
pub mod domain;
pub mod error;
pub mod rgb;

use std::str::FromStr;

use crate::context::STATE;
use crate::domain::{request::UpdateKeyRequest, response::NetworkResponse, Metadata};
use crate::error::WalletError;

use base::utils::validate_network;
use candid::Principal;
use ic_cdk::export_candid;

#[ic_cdk::init]
fn init(network: String, steward_canister: String, key_name: String) {
    ic_wasi_polyfill::init(&[0u8; 32], &[]);

    STATE.with(|m| {
        let mut state = m.borrow_mut();
        let metadata = &mut state.metadata;
        metadata
            .set(Metadata {
                network: validate_network(&network),
                steward_canister: Principal::from_str(&steward_canister)
                    .expect("Failed to parse steward canister id"),
                key_name,
                ..Default::default()
            })
            .expect("Failed to init network");

        state.controllers.insert(ic_caller(), ic_time())
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
