use bitcoin::{Address, EcdsaSighashType};

pub mod constants;
pub mod error;
pub mod tx;
pub mod utils;

pub const SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;

pub struct UserWallet {
    pub address: Address,
    pub derivation_path: Vec<Vec<u8>>,
}
