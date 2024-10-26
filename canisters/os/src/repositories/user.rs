use std::borrow::BorrowMut;

use candid::Principal;

use crate::{context::STATE, domain::UserInfo, error::Error};

pub(crate) fn create_user(user: UserInfo) -> Result<bool, Error> {
    STATE.with_borrow_mut(|s| {
        // let mut state = s.borrow_mut();
        let users = &mut s.user_info;
        if users.contains_key(&user.user_id) {
            Err(Error::UserExists {
                user_id: user.user_id,
            })
        } else {
            let user_invite_code = &mut s.user_invite_code;
            user_invite_code.insert(user.invite_code.clone(), user.user_id);
            users.insert(user.user_id, user);
            // let current_users = s.user_counter.get();
            // let _ = s.user_counter.set(*current_users+1);
            Ok(true)
        }
    })
}
pub(crate) fn get_user_info(user_id: Principal) -> Option<UserInfo> {
    // STATE.with(|s: &std::cell::RefCell<crate::context::State>| {
    //     let state = s.borrow();
    //     let users = state.user_info.borrow();
    //     users.get(&user_id)
    // })
    STATE.with_borrow(|s| s.user_info.get(&user_id))
}

pub(crate) fn delete_user_info(user_id: Principal) {
    STATE.with_borrow_mut(|s| {
        // let state = s.borrow();
        let users = &mut s.user_info;
        users.remove(&user_id);
    })
}

pub(crate) fn update_user_info(user: UserInfo) -> Result<bool, Error> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let users = state.user_info.borrow_mut();
        if !users.contains_key(&user.user_id) {
            Err(Error::UserNotExists {
                user_id: user.user_id,
            })
        } else {
            users.insert(user.user_id, user);
            Ok(true)
        }
    })
    // STATE.with_borrow_mut(|s| {
    //     if s.user_info.contains_key(&user.user_id) {
    //         s.user_info.insert(user.user_id, user);
    //         Ok(true)
    //     } else {
    //         Err(Error::UserNotExists { user_id: user.user_id })
    //     }
    // })
}

pub(crate) fn get_user_seq() -> Result<u128, Error> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let current_counter = *state.user_counter.get();
        state
            .user_counter
            .set(current_counter + 1)
            .map_err(|e| Error::StableSetError {
                msg: format!("{e:?}"),
            })
    })
}

pub(crate) fn get_user_by_invite_code(code: String) -> Option<Principal> {
    STATE.with(|s| {
        let state = s.borrow();
        state.user_invite_code.get(&code)
    })
}

pub(crate) fn set_user_invite_code(code: String, user_id: Principal) -> Result<bool, Error> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let user_code_map = state.user_invite_code.borrow_mut();
        if user_code_map.contains_key(&code) {
            Err(Error::InviteCodeHasExists { code })
        } else {
            user_code_map.insert(code, user_id);
            Ok(true)
        }
    })
}

pub(crate) fn get_total_user_count() -> u128 {
    STATE.with(|s| {
        let state = s.borrow();
        state.user_counter.get().to_owned()
    })
}

pub(crate) fn add_image_link(image_link: String) {
    STATE.with_borrow_mut(|s| {
        let image_link_store = &mut s.user_image_link;
        image_link_store.insert(image_link.len() as u128, image_link);
        // state.user_counter.get().to_owned()
    })
}

pub(crate) fn get_image_link() -> Vec<String> {
    STATE.with_borrow(|s| {
        // let image_link_store = s.user_image_link;
        s.user_image_link.iter().map(|(_, w)| w).collect()
    })
}
