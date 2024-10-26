use candid::Principal;

use crate::repositories;

pub fn serve(user_id: Principal, wallet_address: String) -> bool {
    repositories::wallet_bind::check_wallet_bind_user(user_id, wallet_address)
}
