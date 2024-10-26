mod add_reward;
pub mod create_stake_record;
mod get_btc_price;
mod get_config;
mod get_leader_board;
mod get_leader_board_status;
pub mod get_point;
mod get_task_record;
mod get_timer_settings;
mod get_user_box_reward;
mod get_user_boxes;
mod get_user_invite_reward;
mod get_user_reward;
mod get_user_stake_reward;
mod get_user_stat;
mod init_user;
mod init_user_stat;
mod open_all_box;
mod open_one_box;
// mod save_points_bak;
mod admin;
mod get_stake_leader_board;
mod nft_reward;
mod schedule_task;
mod set_steward_canister;
mod set_timer_settings;
mod update_btc_price;
mod update_config;
mod update_leader_board;
mod update_user_stat;
mod update_user_statke_reward;

mod confirm_staking_record;
mod get_all_user_reward;
mod get_all_user_stake_reward;

use crate::domain::request::{NftRecordRequest, StakeRecordRequest, TimerSettingsRequest};
use crate::domain::response::BoxRecordResponse;
use crate::domain::{
    BoxRewardRecord, BoxStatus, ConfigSetting, InviteRewardRecord, LeaderBoardStatus,
    NftRewardRecord, StakeRewardRecord, TimerSettings, UserStat,
};
use crate::repositories;
// use crate::repositories::leaderboard::update_leader_board;
use crate::{
    context::STATE,
    domain::{request::InitPointArgument, BoxRecord, Metadata, PriceRecord, RewardRecord},
    error::Error,
};
use candid::Principal;
use ic_cdk::{
    api::{is_controller, management_canister::main::CanisterId},
    export_candid, init, update,
};
// use ic_cdk_timers::TimerId;
// use wallet::macros;

use std::sync::atomic::{AtomicU64, Ordering};

use wallet::domain::user::UserType;
use wallet::{domain::reward::Reward, utils::ic_caller};

// extern crate  wallet;

static COUNTER: AtomicU64 = AtomicU64::new(0);

static WALLET_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Set steward canister with given canister id
#[update]
fn set_steward_canister(steward_canister: CanisterId) -> String {
    if is_controller(&ic_cdk::caller()) {
        set_steward_canister::serve(steward_canister)
    } else {
        "UnAuthorized".to_string()
    }
}

#[update]
fn open_one_box(user_id: Principal, box_id: u64) -> Result<BoxRecordResponse, Error> {
    open_one_box::serve(user_id, box_id)
}

#[update]
fn open_all_box(user_id: Principal) -> Result<BoxRecordResponse, Error> {
    open_all_box::serve(user_id)
}

#[update]
async fn add_reward(reward: Reward) -> Result<bool, String> {
    let caller = ic_caller();
    let metadata = get_metadata();
    log!(format!("reward {0}", caller.to_string()));
    if caller == metadata.os_canister {
        log!("point add reward 1");
        add_reward::serve(reward).await;
        Ok(true)
    } else {
        Err("".to_string())
    }
}

#[update]
fn update_timer_config(req: TimerSettingsRequest) -> Result<TimerSettings, String> {
    if is_controller(&ic_cdk::caller()) {
        set_timer_settings::serve(req)
    } else {
        Err("UnAuthorized".to_string())
    }
}

#[update]
fn update_config(conf: ConfigSetting) {
    if is_controller(&ic_cdk::caller()) {
        update_config::serve(conf);
    }
}

#[update]
fn init_user(user: (Principal, UserType)) -> bool {
    let caller = ic_caller();
    let metadata: Metadata = get_metadata();
    // init_user::serve(user.0, user.1, metadata.network);
    if caller == metadata.os_canister || is_controller(&caller) {
        init_user::serve(user.0, user.1, metadata.network);
        true
    } else {
        false
    }
}
// #[update]
// fn reschedule() {
//     if is_controller(&ic_cdk::caller()) {
//         schedule_task_bak::serve();
//     }
// }

#[update]
fn add_user_point(user_id: Principal, point: u64) -> bool {
    let caller = ic_caller();
    if is_controller(&caller) {
        repositories::reward_record::add_user_point(user_id, point)
    } else {
        false
    }
}
#[update]
fn update_user_stake_reward(stake_request:StakeRecordRequest) -> bool {
    let caller = ic_caller();
    if is_controller(&caller) || admin::is_admin(caller) {
        update_user_statke_reward::serve(stake_request.user_id,stake_request.sat)
    } else {
        false
    }
}
#[update]
pub fn add_admin(user_id: Principal) -> bool {
    let caller = ic_caller();
    if is_controller(&caller) {
        admin::add_admin(user_id)
    } else {
        false
    }
}
#[update]
pub fn remove_admin(user_id: Principal) -> bool {
    let caller = ic_caller();
    if is_controller(&caller) {
        admin::remove_admin(user_id)
    } else {
        false
    }
}
#[update]
pub fn update_nft_reward(nft_request: NftRecordRequest) -> bool {
    let caller = ic_caller();
    if is_controller(&caller) || admin::is_admin(caller){
        nft_reward::update_nft_reward(nft_request)
    } else {
        false
    }
}
#[update]
pub fn update_all_nft_reward(nft_requests: Vec<NftRecordRequest>) -> bool {
    let caller = ic_caller();
    if is_controller(&caller) || admin::is_admin(caller){
        nft_reward::update_all_nft_reward(nft_requests);
        true
    } else {
        false
    }
}

#[update]
pub async fn confirm_stake_record(pool_canister:Principal) -> Result<bool, Error> {
    let caller = ic_caller();
    if is_controller(&caller) || admin::is_admin(caller){
        confirm_staking_record::serve(pool_canister).await
    
    } else {
        Err(Error::UnAuthorized(caller.to_text()))
    }
}

#[ic_cdk::query]
pub fn get_all_user_stake_nft_reward() -> Vec<NftRewardRecord> {
    nft_reward::get_all_user_stake_nft_reward()
}

#[ic_cdk::query]
pub fn get_user_stake_nft_reward(user_id: Principal) -> Option<NftRewardRecord> {
    nft_reward::get_user_stake_nft_reward(user_id)
}

#[ic_cdk::query]
pub fn get_all_admins() -> Vec<Principal> {
    admin::get_all_admins()
}
#[ic_cdk::query]
pub fn is_admin(user_id: Principal) -> bool {
    admin::is_admin(user_id)
}

#[ic_cdk::query]
pub fn get_all_user_reward() -> Vec<RewardRecord> {
    get_all_user_reward::serve()
}

#[ic_cdk::query]
pub fn get_all_user_stake_reward() -> Vec<StakeRewardRecord> {
    get_all_user_stake_reward::serve()
}

#[ic_cdk::query]
fn counter() -> u64 {
    // log!("222");
    COUNTER.load(Ordering::Relaxed)
}

#[ic_cdk::query]
fn wallet_counter() -> u64 {
    WALLET_COUNTER.load(Ordering::Relaxed)
}

#[ic_cdk::query]
fn get_metadata() -> Metadata {
    repositories::metadata::get_metadata()
}

#[ic_cdk::query]
fn get_btc_price() -> PriceRecord {
    get_btc_price::serve()
}

#[ic_cdk::query]
fn get_user_reward(user_id: Principal) -> u64{
    get_user_reward::serve(user_id)
}
#[ic_cdk::query]
fn get_user_box_reward(user_id: Principal) -> Option<BoxRewardRecord> {
    get_user_box_reward::serve(user_id)
}

#[ic_cdk::query]
fn get_user_stake_reward(user_id: Principal) -> Option<StakeRewardRecord> {
    get_user_stake_reward::serve(user_id)
}

#[ic_cdk::query]
fn get_user_invite_reward(user_id: Principal) -> Option<InviteRewardRecord> {
    get_user_invite_reward::serve(user_id)
}

#[ic_cdk::query]
fn get_user_close_boxes(user_id: Principal) -> Option<Vec<BoxRecord>> {
    get_user_boxes::serve(user_id, BoxStatus::Close)
}

#[ic_cdk::query]
fn get_user_open_boxes(user_id: Principal) -> Option<Vec<BoxRecord>> {
    get_user_boxes::serve(user_id, BoxStatus::Open)
}

#[ic_cdk::query]
fn get_leader_board() -> Vec<RewardRecord> {
    get_leader_board::serve()
}

#[ic_cdk::query]
fn get_stake_leader_board() -> Vec<StakeRewardRecord> {
    get_stake_leader_board::serve()
}

#[ic_cdk::query]
fn get_leader_board_status() -> LeaderBoardStatus {
    get_leader_board_status::serve()
}

#[ic_cdk::query]
fn get_config() -> ConfigSetting {
    get_config::serve()
}

#[ic_cdk::query]
fn get_timer_settings() -> TimerSettings {
    get_timer_settings::serve()
}

#[ic_cdk::query]
fn get_user_stat() -> Vec<UserStat> {
    get_user_stat::serve()
}

// #[ic_cdk::query]
// fn get_task_record() -> Vec<TimerId> {
//     get_task_record::serve()
// }

// use candid::Principal;
// use serde_bytes::ByteBuf;
// #[ic_cdk::update]
// async fn  get_p(w:String) -> String {
//     let metadata = get_metadata();
//     let sender_principal = match metadata.siwb_canister {
//         Some(c) =>{
//             let wallet_principal_result: Result<(Result<ByteBuf, String>,), _> =  ic_cdk::call(c, "get_principal", ((w.clone()),))
//                 .await;
//             match wallet_principal_result {
//                 Ok(e) =>{
//                      match  e.0 {
//                         Ok(ee)=>{

//                             Principal::from_slice(ee.as_slice())
//                         }
//                         Err(ee)=>{
//                             ic_cdk::print(format!("get siwb address1:{}",ee));
//                             Principal::anonymous()
//                          }
//                      }
//                 }
//                 Err(e)=>{
//                     ic_cdk::print(format!("get siwb address1:{}",e.1));
//                     Principal::anonymous()
//                 }
//             }

//         }
//         None=>Principal::anonymous()
//     };
//     // ic_cdk::print(format!("ic swib:{}",sender_principal.clone().to_string()));
//     sender_principal.clone().to_string()
// }

#[init]
async fn init(args: InitPointArgument) {
    log!(format!("init point:{args:?}"));
    STATE.with(|s| {
        let state = &mut s.borrow_mut();
        state
            .metadata
            .set(Metadata {
                network: args.network,
                steward_canister: args.steward_canister,
                os_canister: args.os_canister,
                siwb_canister: args.siwb_canister,
                updated_time: 0,
            })
            .expect("Failed to init metadata of os canister");
        state
            .leader_board_status
            .set(LeaderBoardStatus::default())
            .expect("Failed to init leaderboard ");
    });
    //init user stat
    // init_user_stat::serve();
    // args.task_period
    schedule_task::serve();
}

#[ic_cdk::post_upgrade]
async fn post_upgrade(args: InitPointArgument) {
    init(args).await;
}

export_candid!();
