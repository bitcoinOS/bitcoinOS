use crate::domain::BindWalletStatus;
use crate::error::Error;
use crate::{
    domain::{request::BindRequest, WalletBindInfo},
    repositories,
};
use wallet::utils::ic_time;

pub fn serve(bind_request: BindRequest) -> Result<bool, Error> {
    let caller = ic_cdk::caller();
    let wallet_info = WalletBindInfo {
        user_id: caller,
        wallet_address: bind_request.wallet_address.to_text(),
        wallet_account: bind_request.account,
        wallet_type: bind_request.wallet_type,
        bind_status: BindWalletStatus::Binded,
        unbind_time: 0,
        bind_time: ic_time(),
    };
    repositories::wallet_bind::add_bind_wallet(wallet_info)
}
