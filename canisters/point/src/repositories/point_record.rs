use crate::{context::STATE, domain::{Metadata,PointRecord,UserStakePool}, error::Error};

use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::{bitcoin::{BitcoinNetwork,Satoshi}, main::CanisterId};

pub(crate) fn get_point_records()-> Vec<PointRecord>{

    STATE.with_borrow(|s| s.point_records.iter().map(|(_, r)| r).collect())

}

pub (crate) fn add_point_record(user:Principal,stake_pool:CanisterId,point:u64){
    STATE.with_borrow_mut(|s| {
        let records = &mut s.point_records;
        let key = UserStakePool{
            user:user,
            stake_pool:stake_pool
        };
        if records.contains_key(&key) {
           let user_point = *records.get(&key)
           user_point
        } else {
            records.insert(key, record.to_owned());
            Ok(())
        }
    })
}