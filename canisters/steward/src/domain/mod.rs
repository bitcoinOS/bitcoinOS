use candid::{CandidType, Decode, Deserialize, Encode};
use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;
use ic_stable_structures::{storable::Bound, Storable};

use crate::ICBitcoinNetwork;

// #[derive(Debug, Clone, CandidType, Deserialize, Default)]
// pub struct EcdsaKey {
//     pub key: EcdsaKeyId,
//     pub updated_time: u64,
// }

// impl Storable for EcdsaKey {
//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         std::borrow::Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

#[derive(Debug, Clone, CandidType, Deserialize, Default)]
pub struct Metadata {
    pub network: ICBitcoinNetwork,
    pub ecdsa_key_id: EcdsaKeyId,
    pub updated_time: u64,
}

impl Storable for Metadata {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
