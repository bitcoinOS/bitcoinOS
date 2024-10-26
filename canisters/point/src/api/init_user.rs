use candid::Principal;
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use wallet::domain::user::UserType;

use crate::repositories;

pub fn serve(user_id: Principal, user_type: UserType, network: BitcoinNetwork) {
    ic_cdk::print("init 1");
    repositories::init::init(user_id, user_type, network)
}
