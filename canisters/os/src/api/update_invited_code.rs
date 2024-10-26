use candid::Principal;

use crate::{domain::UserInfo, error::Error, repositories};
pub fn serve(user_id: Principal, code: String) -> Result<bool, Error> {
    let user_res = repositories::user::get_user_info(user_id);
    match user_res {
        Some(u) => {
            if u.invite_code != code {
                if u.invited_code.is_none() {
                    let new_user = UserInfo {
                        invited_code: Some(code),
                        ..u
                    };
                    repositories::user::update_user_info(new_user)
                } else {
                    Err(Error::InvitedCodeHasExists { user_id: u.user_id })
                }
            } else {
                Err(Error::InviteCodeError { code })
            }
        }
        None => Err(Error::UserNotExists { user_id }),
    }
}
