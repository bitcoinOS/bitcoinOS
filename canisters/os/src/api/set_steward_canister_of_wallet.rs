use ic_cdk::api::management_canister::main::CanisterId;

use crate::error::Error;

pub(super) async fn serve(
    wallet_canister: CanisterId,
    steward_canister: CanisterId,
) -> Result<String, Error> {
    let resp: Result<(String,), _> =
        ic_cdk::call(wallet_canister, "set_steward_canister", (steward_canister,)).await;

    resp.map(|(r,)| r).map_err(|e| Error::WalletNotFound(e.1))
}
