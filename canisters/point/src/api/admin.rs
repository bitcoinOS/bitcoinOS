use candid::Principal;

use crate::repositories;

pub fn get_all_admins() -> Vec<Principal> {
    repositories::point_admin::get_all_admins()
}

pub fn is_admin(user_id: Principal) -> bool {
    repositories::point_admin::is_admin(user_id)
}

pub fn add_admin(user_id: Principal) -> bool {
    repositories::point_admin::add_admin(user_id)
}

pub fn remove_admin(user_id: Principal) -> bool {
    repositories::point_admin::remove_admin(user_id)
}
