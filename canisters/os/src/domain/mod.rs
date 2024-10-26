pub mod request;
pub mod response;

use std::{borrow::Cow, collections::HashMap};

use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, main::CanisterId};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;
// use std::collections::VecDeque;
use wallet::{
    domain::{user::UserType, wallet::DBankWalletInfo},
    utils::time_to_day,
};

use crate::constants::{DEFAULT_CYCLES_PER_CANISTER, DEFAULT_CYCLES_PER_DBANK, MAX_BIND_WALLET};

// use ic_stable_structures::BTreeMap as StableBTreeMap;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Metadata {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub wallet_cycles: u64,
    pub dbank_cycles: Option<u64>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            steward_canister: Principal::anonymous(),
            network: BitcoinNetwork::Regtest,
            wallet_cycles: DEFAULT_CYCLES_PER_CANISTER,
            dbank_cycles: Some(DEFAULT_CYCLES_PER_DBANK),
        }
    }
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

/// The `State` will store the canister info when a user create a wallet.
/// A wallet is also a canister, call `SmartWallet`
#[derive(Debug, CandidType, Deserialize, Clone)]
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

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CandidType, Deserialize)]
pub struct WalletInfoKey {
    pub owner: Principal,
    pub wallet_canister: CanisterId,
}

impl Storable for WalletInfoKey {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

/// The information of a wallet
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct WalletInfo {
    pub name: String,
    pub owner: Principal,
    pub wallet_canister: CanisterId,
    pub bitcoin_address: String,
    pub network: BitcoinNetwork,
    pub steward_canister: CanisterId,
    pub created_at: u64,
    pub mode: Option<WalletMode>,
}

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WalletMode {
    Idenpendent,
    Shared,
}

impl WalletInfo {
    pub fn is_share_wallet(&self) -> bool {
        match self.mode.as_ref() {
            None => false,
            Some(m) => m == &WalletMode::Shared,
        }
    }
}

impl From<DBankWalletInfo> for WalletInfo {
    fn from(value: DBankWalletInfo) -> Self {
        Self {
            name: value.name,
            owner: value.owner,
            wallet_canister: value.dbank_canister,
            bitcoin_address: value.bitcoin_address,
            network: value.network,
            steward_canister: value.steward_canister,
            created_at: value.created_at,
            mode: Some(WalletMode::Shared),
        }
    }
}

impl Storable for WalletInfo {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

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

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum WalletType {
    SelfCustody,
    Shared,
    Unisat,
    Wizz,
}

impl WalletType {
    pub fn list_wallet_types() -> Vec<String> {
        vec![
            "SelfCustody".to_string(),
            "Shared".to_string(),
            "Unisat".to_string(),
            "Wizz".to_string(),
        ]
    }
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct DBankInfo {
    // The id in os, start from 10001
    pub dbank_id: u64,
    pub name: String,
    pub owner: Principal,
    pub dbank_canister: CanisterId,
    pub network: BitcoinNetwork,
    pub steward_canister: CanisterId,
    pub status: DBankStatus,
    pub start_seq_in_os: u64,
    pub current_seq_in_os: u64,
    pub max_wallet_limit: u32,
    pub created_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum DBankStatus {
    Activing,
    Inactive,
    Forbbiden,
}

impl DBankInfo {
    pub fn is_full(&self) -> bool {
        (self.current_seq_in_os - self.start_seq_in_os) as u32 >= self.max_wallet_limit
    }
}

impl Storable for DBankInfo {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

/// canister info will be stored in stable storage
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct CanisterModuleInfo {
    pub canister_name: String,
    pub canister_id: CanisterId,
    pub created_at: u64,
}

impl Storable for CanisterModuleInfo {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum UserStatus {
    Forbbiden,
    Active,
    Inactive,
}

/// The information of a user
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct UserInfo {
    pub seq: u128,
    pub name: Option<String>,
    pub user_id: Principal,
    pub user_desc: Option<String>,
    pub user_img: Option<String>,
    pub user_type: UserType,
    pub wallet_address: Option<String>,
    pub primary_wallet: Option<Principal>,
    pub network: BitcoinNetwork,
    pub invite_code: String,
    pub invited_code: Option<String>,
    pub user_status: UserStatus,
    pub last_login_at: u64,
    pub last_reward_at: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Storable for UserInfo {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

/// The information of a user
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct LoginLog {
    pub user_id: Principal,
    pub login_at: u64,
}

impl Storable for LoginLog {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

/// user bind wallet
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct WalletBindInfo {
    // pub bind_id: u64,
    pub user_id: Principal,
    pub wallet_address: String,
    pub wallet_account: Option<Vec<u8>>,
    pub wallet_type: BindWalletType,
    pub bind_time: u64,
    pub unbind_time: u64,
    pub bind_status: BindWalletStatus,
}

impl Default for WalletBindInfo {
    fn default() -> Self {
        Self {
            // bind_id: 0,
            user_id: Principal::anonymous(),
            wallet_address: "".to_string(),
            wallet_account: None,
            wallet_type: BindWalletType::ICP,
            bind_time: 0,
            bind_status: BindWalletStatus::Unknown,
            unbind_time: 0,
        }
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum BindWalletType {
    ICP,
    BTC,
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum BindWalletStatus {
    Unknown,
    Binded,
    Unbind,
}

impl Storable for WalletBindInfo {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

// #[derive(Debug, CandidType, Deserialize, Clone)]
// pub struct UserBindCount {
//     pub user_id: Principal,
//     pub bind_id: u64,
// }

// impl Storable for UserBindCount {
//     fn from_bytes(bytes: Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }

//     fn to_bytes(&self) -> Cow<[u8]> {
//         Cow::Owned(Encode!(self).unwrap())
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CandidType, Deserialize)]
pub struct UserBindKey {
    pub user_id: Principal,
    pub bind_id: u64,
}

impl Storable for UserBindKey {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

// #[derive(Debug, CandidType, Deserialize, Clone, Default)]
// pub struct LogBuffer {
//     max_capacity: usize,
//     entries: VecDeque<HashMap<u64, HashMap<Principal, LoginLog>>>,
// }
// /*
// |
// |day1
// |    |
//      |u1
//      |u2
// |day2|u2
// |    |u3
// */
// impl LogBuffer {
//     pub fn with_capacity(max_capacity: usize) -> Self {
//         Self {
//             max_capacity,
//             entries: VecDeque::with_capacity(max_capacity),
//         }
//     }

//     /// Adds a new entry to the buffer, potentially evicting older entries.
//     pub fn append(&mut self, entry: LoginLog) {
//         let mut logs = self.entries.pop_back().unwrap();
//         let day = time_to_day(entry.login_at);
//         if logs.contains_key(&day) {
//             let day_logs = logs.get_mut(&day).unwrap();
//             day_logs.insert(entry.user_id, entry);
//         } else {
//             let mut user_action = HashMap::new();
//             user_action.insert(entry.user_id, entry);
//             let mut day_log = HashMap::new();
//             day_log.insert(day, user_action);
//             if self.entries.len() >= self.max_capacity {
//                 self.entries.pop_front();
//             }
//             self.entries.push_back(day_log);
//         }
//     }
// }
// impl Storable for LogBuffer {
//     fn from_bytes(bytes: Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }

//     fn to_bytes(&self) -> Cow<[u8]> {
//         Cow::Owned(Encode!(self).unwrap())
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

#[cfg(test)]
mod tests {
    use wallet::domain::staking::PoolStatus;

    #[test]
    fn pool_status_display_should_works() {
        let inactive = PoolStatus::Inactive;

        let expected_status = "inactive";

        assert_eq!(inactive.to_string(), expected_status);

        let activing = PoolStatus::Activing;

        let expected_status = "activing";

        assert_eq!(activing.to_string(), expected_status);

        let suspended = PoolStatus::Suspended;

        let expected_status = "suspended";

        assert_eq!(suspended.to_string(), expected_status);

        let terminated = PoolStatus::Terminated;

        let expected_status = "terminated";

        assert_eq!(terminated.to_string(), expected_status);

        let completed = PoolStatus::Completed;

        let expected_status = "completed";

        assert_eq!(completed.to_string(), expected_status);
    }

    #[test]
    fn pool_status_from_string_should_works() {
        let inactive = "Inactive".to_string();
        let activing = "activing".to_string();
        let suspended = "Suspended".to_string();
        let terminated = "terminated".to_string();
        let completed = "Completed".to_string();
        let unknown = "unknown".to_string();

        assert_eq!(PoolStatus::from(inactive), PoolStatus::Inactive);
        assert_eq!(PoolStatus::from(activing), PoolStatus::Activing);
        assert_eq!(PoolStatus::from(suspended), PoolStatus::Suspended);
        assert_eq!(PoolStatus::from(terminated), PoolStatus::Terminated);
        assert_eq!(PoolStatus::from(completed), PoolStatus::Completed);
        assert_eq!(PoolStatus::from(unknown), PoolStatus::Inactive);
    }
}
