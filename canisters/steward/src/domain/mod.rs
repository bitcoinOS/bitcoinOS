use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};

use crate::{
    constants::{ECDSA_SIZE, METADATA_SIZE},
    ICBitcoinNetwork,
};

#[derive(Debug, Clone, CandidType, Deserialize, Default)]
pub struct ECDSAKey {
    pub key: String,
    pub updated_time: u64,
}

impl Storable for ECDSAKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: ECDSA_SIZE as u32,
        is_fixed_size: false,
    };
}

#[derive(Debug, Clone, CandidType, Deserialize, Default)]
pub struct Metadata {
    pub network: ICBitcoinNetwork,
}

impl Storable for Metadata {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: METADATA_SIZE as u32,
        is_fixed_size: false,
    };
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UpdateKeyRequest {
    pub new_key: String,
    pub old_key: String,
}
