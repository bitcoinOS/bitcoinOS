use crate::repositories;

pub fn serve() -> u64 {
    repositories::wallet_info::count()
}
