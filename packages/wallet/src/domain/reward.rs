use candid::{CandidType, Decode, Encode, Principal};

use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Reward {
    pub user_id: Principal,
    pub reward_type: RewardType,
    pub reward_status: RewardStaus,
    pub create_time: u64,
    pub invite_user: Option<Principal>,
    pub twiter: Option<String>,
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RewardStaus {
    Confirmed,
    Init,
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RewardType {
    Login,
    Invite,
    Share,
    Bind,
    Stake,
    RedPacket,
}

impl Storable for Reward {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RewardMode {
    Fixed,
    Random,
}
