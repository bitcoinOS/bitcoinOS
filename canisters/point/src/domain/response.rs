use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct BoxRecordResponse {
    pub user_id: Principal,
    pub open_count: u64,
    pub og_count: u64,
    pub fund_count: u64,
    pub boost_card_count: u64,
    pub box_point: u64,
}

impl Default for BoxRecordResponse {
    fn default() -> Self {
        Self {
            user_id: Principal::anonymous(),
            og_count: 0,
            fund_count: 0,
            boost_card_count: 0,
            open_count: 0,
            box_point: 0,
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PointResponse {
    pub user_id: Principal,
    pub point: u64,
    pub point_type: u64,
}