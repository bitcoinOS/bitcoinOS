pub mod api;
pub mod constants;
pub mod context;
pub mod domain;
pub mod error;

pub type ICBitcoinNetwork = ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
pub type PublicKey = Vec<u8>;
