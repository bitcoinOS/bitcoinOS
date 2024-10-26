use candid::CandidType;
use serde::Deserialize;
#[derive(Clone, Copy, Debug, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum UserType {
    II,
    Wallet,
}

impl Default for UserType {
    fn default() -> Self {
        Self::II
    }
}
