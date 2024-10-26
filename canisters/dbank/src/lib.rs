pub mod api;
pub mod constants;
pub mod context;
pub mod domain;
pub mod error;
pub mod repositories;
pub mod rgb;

use crate::context::STATE;
use crate::domain::{
    request::CreateDBankWalletRequest,
    response::{NetworkResponse, PublicKeyResponse},
    DBankWalletInfo, Metadata, TransactionLog, WalletOperationEvent,
};
use crate::error::DBankError;

use candid::{CandidType, Principal};
use constants::DAILY_LIMIT_SATOSHI;
use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, MillisatoshiPerByte, Satoshi};
use ic_cdk::api::management_canister::main::CanisterId;
use ic_cdk::export_candid;

use serde::Deserialize;
use wallet::domain::request::{StakingRequest, TransferRequest, UtxosRequest};
use wallet::domain::response::UtxosResponse;

use wallet::domain::EcdsaKeyIds;
use wallet::utils::{check_normal_principal, ic_caller, ic_time};

/// Create a wallet when init the wallet canister
#[ic_cdk::init]
async fn init(args: InitDBankArgument) {
    ic_wasi_polyfill::init(&[0u8; 32], &[]);

    let caller = ic_caller();
    check_normal_principal(caller).expect("user should be a normal principal");

    let owner = match args.owner {
        Some(o) => o,
        None => caller,
    };

    let name = args.name;
    let network = args.network;
    let steward_canister = args.steward_canister;
    let ecdsa_key_id = EcdsaKeyIds::from(network).to_key_id();
    let updated_time = ic_time();
    let dbank_canister = ic_cdk::id();

    STATE.with(|s| {
        let metadata = &mut s.borrow_mut().metadata;
        metadata
            .set(Metadata {
                name,
                owner,
                network,
                dbank_canister,
                steward_canister,
                ecdsa_key_id,
                daily_limit_satoshi: DAILY_LIMIT_SATOSHI,
                dbank_id: args.dbank_id,
                start_seq_in_os: args.seq_in_os,
                current_seq_in_os: args.seq_in_os,
                max_wallet_limit: args.max_wallet_limit,
                updated_time,
            })
            .expect("Failed to init metadata")
    });
}

// #[ic_cdk::update]
// fn issue_rgb20() -> String {
//     rgb::issue_rgb20()
// }

// Load timer ids from stable storage after canister upgrade
// #[ic_cdk::post_upgrade]
// fn post_upgrade() {
//     let staking_records = repositories::staking_record::list_staking().iter().filter(|r| r.tx)
// }

export_candid!();

#[derive(CandidType, Deserialize)]
struct InitDBankArgument {
    dbank_id: u64,
    name: String,
    network: BitcoinNetwork,
    steward_canister: Principal,
    seq_in_os: u64,
    max_wallet_limit: u32,
    owner: Option<Principal>,
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
