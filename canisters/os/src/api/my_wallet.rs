use candid::Principal;

use crate::{domain::WalletInfo, repositories};

pub(super) fn serve(owner: Principal) -> Vec<WalletInfo> {
    repositories::wallet_info::find_info_by_owner(owner)
}
