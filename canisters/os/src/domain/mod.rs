use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

const WALLET_OWNER_MAX_SIZE: u32 = 100;
const WALLET_ACTION_MAX_SIZE: u32 = 100;

/// The `State` will store the canister info when a user create a wallet.
/// A wallet is also a canister, call `SmartWallet`
#[derive(Debug, CandidType, Deserialize)]
pub struct WalletOwner {
    pub canister_id: Principal,
    pub owner: Principal,
    pub created_at: u64,
}

/// For a type to be used in Stable storage like `StableBtreeMap`, it need to implement the `Storable` trait,
/// which specifies how the type can be serialized/deserialized.
impl Storable for WalletOwner {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: WALLET_OWNER_MAX_SIZE,
        is_fixed_size: false,
    };
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Metadata {}

#[derive(CandidType, Deserialize)]
pub struct WalletAction {
    pub operator: Principal,
    pub action: Action,
    pub op_time: u64,
}

#[derive(CandidType, Deserialize)]
pub enum Action {
    Create,
    Delete,
}

impl Storable for WalletAction {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: WALLET_ACTION_MAX_SIZE,
        is_fixed_size: false,
    };
}

pub struct WalletCanisterDeployArgs {
    // sub_account: Option<Subaccount>,
}
