use crate::{domain::UserInfo, repositories};
use candid::Principal;
pub fn serve(user_id: Principal) -> Option<UserInfo> {
    repositories::user::get_user_info(user_id)
}
