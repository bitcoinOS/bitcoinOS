use crate::{
    domain::{RawWallet, SelfCustodyKey},
    repositories,
};

pub(super) fn serve() -> Vec<(SelfCustodyKey, RawWallet)> {
    repositories::wallet::list_wallet()
}
