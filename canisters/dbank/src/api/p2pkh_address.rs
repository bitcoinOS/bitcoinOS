use candid::Principal;

use crate::{domain::Metadata, repositories::wallet};

/// Returns the P2PKH address of given owner
pub(super) fn serve(metadata: &Metadata, wallet_owner: Principal) -> Option<String> {
    wallet::get_p2pkh_wallet(metadata, wallet_owner).map(|w| w.address.to_string())
}
