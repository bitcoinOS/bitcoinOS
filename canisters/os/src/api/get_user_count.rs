use crate::repositories;
pub fn serve() -> u128 {
    repositories::user::get_total_user_count()
}
