use candid::{CandidType, Principal};

use ic_cdk::api::management_canister::bitcoin::Satoshi;
use wallet::domain::staking::StakingRecord;
use crate::{context::STATE, error::Error};
use crate::domain::{StakingPoolInfo,PointRecord};
use crate::error::StakingError;
use crate::repositories::metadata::get_metadata;
use crate::repositories::point_record::save_point_record;
use std::collections::HashMap;
use wallet::utils::ic_time;
// # 获取质押池子列表
// # 遍历池子里的质押记录
// # 计算每个人当前的总质押余额
// # 计算每个人的积分
pub async fn  get_stake_pools(os_canister:Principal)->Vec<StakingPoolInfo>{

    let resp: Result<(Vec<StakingPoolInfo>,), _> =
    ic_cdk::call(os_canister, "list_staking_pool", ((),)).await;
    resp.map(|b|{
        b.0
    }).expect("get stake pool error")
}


pub fn confirm_stake_record() {
     
}

pub async fn serve(){
    ic_cdk::print(format!(
        "in time period \n"
    ));
    let metadata = get_metadata();
    let stake_pools =  get_stake_pools(metadata.os_canister).await;
    let mut total_stake_point_per_user :HashMap<Principal,u64> = HashMap::new();
    
   
    for p in stake_pools{
        ic_cdk::print(format!(
            "in  stake pool {p:?}\n"
        ));
        let resp: Result<(Result<Vec<StakingRecord>, StakingError>,), _> = ic_cdk::call(p.staking_pool_canister,"list_staking",((),)).await;
        resp.map(|b|{
           match b.0{
            Ok(v)=>{
                 for s in v {
                    ic_cdk::print(format!(
                        "in stake record {s:?}\n"
                    ));
                    let sender = s.sender;
                    if let Some(ammout) = total_stake_point_per_user.get(&sender){
                        total_stake_point_per_user.insert(sender.clone(), ammout+s.sent_amount);
                    }else{
                        total_stake_point_per_user.insert(sender.clone(), s.sent_amount);
                    }
                 }
            }
            _=>{}
           };
        }).expect("get stake pool error")
    }
    
    for (user,sat) in total_stake_point_per_user{
        let user_point = PointRecord{
            network: metadata.network,
            staker: user,
            actual_amount:sat,
            points:sat * metadata.point_per_sat,
            updated_time:ic_time()
        };
        save_point_record(user_point);
    }
}