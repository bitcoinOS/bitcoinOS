use crate::error::Error;
use crate::{
    domain::{request::BindRequest, WalletBindInfo},
    repositories,
};
// use wallet::utils::ic_time;

pub fn serve(bind_request: BindRequest) -> Result<bool, Error> {
    let caller = ic_cdk::caller();
    let wallet_info = WalletBindInfo {
        user_id: caller,
        wallet_address: bind_request.wallet_address.to_text(),
        ..Default::default()
    };
    repositories::wallet_bind::remove_bind_wallet(wallet_info)
}
