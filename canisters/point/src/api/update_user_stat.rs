use crate::repositories;

pub fn serve() {
    let user_count = repositories::user_stat::get_user_count();
    repositories::user_stat::update_user_stat(user_count as u128);
}
