use crate::{context::STATE, domain::{PointRecord}, error::Error};

use ic_cdk::api::management_canister::{bitcoin::{BitcoinNetwork,Satoshi}, main::CanisterId};

use wallet::utils::ic_time;
pub(crate) fn get_point_records()-> Vec<PointRecord>{

    STATE.with_borrow(|s| s.point_records.iter().map(|(_, r)| r).collect())

}

pub (crate) fn save_point_record(p:PointRecord){
    STATE.with_borrow_mut(|s| {
        let records = &mut s.point_records;
        let user =  p.staker;
        if records.contains_key(&user) {
           let user_point = records.get(&user).unwrap();
           let new_user_point = PointRecord{
                    network: user_point.network,
                    staker: user,
                    actual_amount:p.actual_amount,
                    points:p.points+user_point.points,
                    updated_time:ic_time()
           };
           records.insert(user, new_user_point);
        } else {
            records.insert(p.staker, p.to_owned());
        }
    })
}
