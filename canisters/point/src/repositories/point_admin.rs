use crate::context::STATE;
use candid::Principal;

use wallet::utils::ic_time;

pub fn get_all_admins() -> Vec<Principal> {
    STATE.with_borrow(|s| s.point_admin.iter().map(|(k, _)| k).collect())
}

pub fn is_admin(user_id: Principal) -> bool {
    STATE.with_borrow(|s| s.point_admin.contains_key(&user_id))
}

pub fn add_admin(user_id: Principal) -> bool {
    STATE.with_borrow_mut(|s| {
        let admins = &mut s.point_admin;
        if admins.contains_key(&user_id) {
            true
        } else {
            admins.insert(user_id, ic_time());
            true
        }
    })
}

pub fn remove_admin(user_id: Principal) -> bool {
    STATE.with_borrow_mut(|s| {
        let admins = &mut s.point_admin;

        if admins.contains_key(&user_id) {
            admins.remove(&user_id);
            true
        } else {
            true
        }
    })
}
