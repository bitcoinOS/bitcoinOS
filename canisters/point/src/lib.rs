pub mod api;
pub mod constants;
pub mod context;
pub mod domain;
pub mod error;
pub mod repositories;

use std::borrow::Cow;

use ic_stable_structures::{storable::Bound, Storable};

/// A helper type implementing Storable for all
/// serde-serializable types using the CBOR encoding.
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
        max_size: 100,
        is_fixed_size: false,
    };
}
