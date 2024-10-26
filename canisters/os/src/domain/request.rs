use crate::domain::UserType;
use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::{
    bitcoin::{BitcoinNetwork, Satoshi},
    main::CanisterId,
};
use serde::Deserialize;

use super::{BindWalletType, DBankInfo};

#[derive(Debug, CandidType, Deserialize)]
pub struct InitArgument {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub wallet_cycles: Option<u64>,
    pub dbank_cycles: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct InitWalletArgument {
    pub name: String,
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub owner: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct InitDBankArgument {
    pub dbank_id: u64,
    pub name: String,
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub seq_in_os: u64,
    pub max_wallet_limit: u32,
    pub owner: Principal,
}

impl From<DBankInfo> for InitDBankArgument {
    fn from(value: DBankInfo) -> Self {
        Self {
            dbank_id: value.dbank_id,
            name: value.name,
            network: value.network,
            steward_canister: value.steward_canister,
            seq_in_os: value.start_seq_in_os,
            max_wallet_limit: value.max_wallet_limit,
            owner: value.owner,
        }
    }
}
#[derive(CandidType, Deserialize, Clone)]
pub struct CreateStakingPoolRequest {
    pub name: String,
    pub description: String,
    pub annual_interest_rate: u16,
    pub duration_in_day: u64,
    pub status: String,
    pub start_time: u64,
    // pub stake_end_time: u64,
    pub end_time: u64,
    pub fund_management: String,
    pub minimum_stake_amount: Option<u64>,
    pub boost_rate: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct RegisterStakingPoolRequest {
    pub name: String,
    pub description: String,
    pub annual_interest_rate: u16,
    pub duration_in_day: u64,
    pub staking_pool_canister: CanisterId,
    pub bitcoin_address: String,
    pub status: String,
    pub start_time: u64,
    // pub stake_end_time: u64,
    pub end_time: u64,
    pub fund_management: String,
    pub minimum_stake_amount: Option<u64>,
    pub boost_rate: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct RegisterWalletRequest {
    pub name: String,
    pub owner: Principal,
    pub wallet_canister: CanisterId,
    pub bitcoin_address: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct RegisterStakingRecordRequest {
    pub txid: String,
    pub sender: Principal,
    pub sender_address: String,
    pub sent_amount: Satoshi,
    pub sent_time: u64,
    pub network: BitcoinNetwork,
    pub staking_canister: CanisterId,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct InstallWalletRequest {
    pub owner: Principal,
    pub wallet_canister: CanisterId,
    pub reinstall: bool,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct ReinstallWalletRequest {
    pub owner: Principal,
    pub wallet_canister: CanisterId,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct UpdateBitcoinAddressRequest {
    pub canister_id: CanisterId,
    pub bitcoin_address: String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct UserRequest {
    pub name: Option<String>,
    pub user_id: Principal,
    pub user_desc: Option<String>,
    pub user_img: Option<String>,
    pub user_type: UserType,
    pub wallet_address: Option<String>,
    pub invited_code: Option<String>,
    pub sign_message: Option<String>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct BindRequest {
    pub wallet_address: Principal,
    pub sig_message: Option<String>,
    pub account: Option<Vec<u8>>,
    pub wallet_type: BindWalletType,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct UserProfileRequest {
    pub user_id: Principal,
    pub image_link: String,
    pub user_name: String,
}

#[cfg(test)]
mod tests {

    use wallet::domain::staking::{InitStakingPoolArgument, PoolStatus, StakingPoolInfo};

    use super::*;

    #[test]
    fn init_staking_pool_argument_should_works() {
        let info = StakingPoolInfo {
            name: "test".to_string(),
            description: "test".to_string(),
            network: BitcoinNetwork::Regtest,
            annual_interest_rate: 10,
            duration_in_day: 10,
            os_canister: Principal::anonymous(),
            steward_canister: Principal::anonymous(),
            status: PoolStatus::Inactive,
            start_time: 1_718_938_706_200_000_000,
            // stake_end_time: 1_750_474_572_000_000_000,
            end_time: 1_750_474_572_000_000_000,
            staking_pool_canister: Principal::anonymous(),
            bitcoin_address: "bcrt1qmyyeh2hxv6yxlahr2mzaqrwgd7kpms4pfrx6jxh30qz7m9nd9ugq0vpmlx"
                .to_string(),
            created_at: 1_718_942_277_876_588_303,
            fund_management: Default::default(),
            boost_rate: Some(100),
            minimum_stake_amount: Some(10000),
        };

        let init_arg = InitStakingPoolArgument {
            name: "test".to_string(),
            description: "test".to_string(),
            network: BitcoinNetwork::Regtest,
            annual_interest_rate: 10,
            duration_in_day: 10,
            os_canister: Principal::anonymous(),
            steward_canister: Principal::anonymous(),
            status: "inactive".to_string(),
            start_time: 1_718_938_706_200_000_000,
            // stake_end_time: 1_750_474_572_000_000_000,
            end_time: 1_750_474_572_000_000_000,
            fund_management: "transfer".to_string(),
            boost_rate: Some(100),
            minimum_stake_amount: Some(10000),
        };

        assert_eq!(InitStakingPoolArgument::from(info), init_arg);
    }
}
