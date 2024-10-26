use candid::Principal;

use crate::error::Error;

pub async fn serve(pool_canister_id: Principal)->Result<bool, Error> {
    let resp: Result<(bool,), _> =
        ic_cdk::call(pool_canister_id, "confirm_staking_record", ((),)).await;
    match resp {
        Ok(res) =>{
            Ok(res.0)
        },
        Err(e) => Err(Error::ConfirmStakeError(pool_canister_id, e.1))
    }
}
