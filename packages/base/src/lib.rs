pub mod constants;
pub mod domain;
pub mod ecdsa;
pub mod error;
pub mod tx;
pub mod utils;

use std::borrow::Cow;

use bitcoin::EcdsaSighashType;
use ic_stable_structures::{storable::Bound, Storable};

pub const SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;

pub type ICBitcoinNetwork = ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;

#[derive(Default)]
struct Cbor<T>(pub T)
where
    T: serde::Serialize + serde::de::DeserializeOwned;

impl<T> std::ops::Deref for Cbor<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Storable for Cbor<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        ciborium::ser::into_writer(&self.0, &mut buf).unwrap();
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(ciborium::de::from_reader(bytes.as_ref()).unwrap())
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1000,
        is_fixed_size: false,
    };
}
