use crate::domain::WalletType;

pub(crate) fn serve() -> Vec<String> {
    WalletType::list_wallet_types()
}
