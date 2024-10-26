use wallet::utils::ic_time;

use crate::{
    domain::{request::UserProfileRequest, UserInfo},
    repositories,
};

use crate::error::Error;

pub fn serve(usrer_profile_req: UserProfileRequest) -> Result<bool, Error> {
    let user = repositories::user::get_user_info(usrer_profile_req.user_id);
    if let Some(u) = user {
        let new_user = UserInfo {
            user_img: Some(usrer_profile_req.image_link),
            name: Some(usrer_profile_req.user_name),
            updated_at: ic_time(),
            ..u
        };
        repositories::user::update_user_info(new_user)
    } else {
        Ok(false)
    }
}
