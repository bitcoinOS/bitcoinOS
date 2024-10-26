use candid::Principal;

use crate::{domain::WalletBindInfo, repositories};

pub fn serve(user_id: Principal) -> Option<Vec<WalletBindInfo>> {
    // let caller = ic_cdk::caller();

    repositories::wallet_bind::get_user_bind_wallets(user_id)
}
