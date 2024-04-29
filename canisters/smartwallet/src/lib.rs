pub mod api;
pub mod constants;
pub mod context;
pub mod domain;
pub mod error;
pub mod repositories;
pub mod rgb;

use crate::context::STATE;
use crate::domain::{
    request::TransferRequest, response::NetworkResponse, response::PublicKeyResponse, Metadata,
    TransactionLog,
};
use crate::error::WalletError;

use candid::{CandidType, Principal};
use constants::DAILY_LIMIET_SATOSHI;
use ic_cdk::api::management_canister::bitcoin::{
    BitcoinNetwork, GetUtxosResponse, MillisatoshiPerByte, Satoshi,
};
use ic_cdk::export_candid;
use serde::Deserialize;
use wallet::domain::EcdsaKeyIds;
use wallet::utils::{ic_caller, ic_time};

/// Create a wallet when init the wallet canister
#[ic_cdk::init]
async fn init(args: InitArgument) {
    ic_wasi_polyfill::init(&[0u8; 32], &[]);

    let owner = ic_caller();
    let name = args.name;
    let network = args.network;
    let steward_canister = args.steward_canister;
    let ecdsa_key_id = EcdsaKeyIds::from(network).to_key_id();
    let updated_time = ic_time();

    STATE.with(|s| {
        let metadata = &mut s.borrow_mut().metadata;
        metadata
            .set(Metadata {
                name,
                owner,
                network,
                steward_canister,
                ecdsa_key_id,
                daily_limit_satoshi: DAILY_LIMIET_SATOSHI,
                updated_time,
            })
            .expect("Failed to init metadata")
    });
}

// #[ic_cdk::update]
// fn issue_rgb20() -> String {
//     rgb::issue_rgb20()
// }

export_candid!();

#[derive(CandidType, Deserialize)]
struct InitArgument {
    name: String,
    network: BitcoinNetwork,
    steward_canister: Principal,
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
