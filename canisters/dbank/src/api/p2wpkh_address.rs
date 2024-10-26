use candid::Principal;

use crate::{domain::Metadata, repositories::wallet};

/// Returns the P2WPKH address of given owner
pub(super) fn serve(metadata: &Metadata, wallet_owner: Principal) -> Option<String> {
    wallet::get_p2wpkh_wallet(metadata, wallet_owner).map(|w| w.address.to_string())
}
