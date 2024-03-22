use crate::{domain::WalletAction, services, WALLET_ACTION};

pub fn serve(idx: u64) -> Option<WalletAction> {
    WALLET_ACTION.with(|w| services::get_wallet_action::execute(w.into(), idx))
}
