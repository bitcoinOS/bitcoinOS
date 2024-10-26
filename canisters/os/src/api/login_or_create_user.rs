use crate::{domain::UserInfo, error::Error, repositories};
use wallet::utils;

pub fn serve(user: UserInfo) -> Result<UserInfo, Error> {
    let user_option = repositories::user::get_user_info(user.user_id);
    match user_option {
        //update
        Some(_) => {
            let login_time = utils::ic_time();

            let new_user = UserInfo {
                last_login_at: login_time,
                ..user
            };

            let update_res = repositories::user::update_user_info(new_user.clone());

            match update_res {
                Ok(_) => Ok(new_user),
                Err(e) => Err(e),
            }
        }
        //create
        None => {
            let create_res = repositories::user::create_user(user.clone());
            match create_res {
                Ok(_) => Ok(user),
                Err(e) => Err(e),
            }
        }
    }
}

// let user = UserInfo{
//     seq:seq,
//     name:user_request.name,
//     user_id:user_request.user_id,
//     user_desc:user_request.user_desc,
//     user_img:user_request.user_img,
//     user_status:UserStatus::Active,
//     user_type:user_request.user_type,
//      wallet_address:
//      network: ,
//      ivite_code: ,
//      ivited_code: Option<String>,
//      user_status: UserStatus,
//      last_login_at: u64,
//      last_login_day: u64,
//      created_at: u64,
//      updated_at: u64,
// }
