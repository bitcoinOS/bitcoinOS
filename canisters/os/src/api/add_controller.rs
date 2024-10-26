use candid::Principal;
use ic_cdk::api::management_canister::main::{
    update_settings, CanisterId, CanisterSettings, UpdateSettingsArgument,
};

use crate::error::Error;

pub(super) async fn serve(
    canister_id: CanisterId,
    controllers: Vec<Principal>,
) -> Result<bool, Error> {
    let arg = UpdateSettingsArgument {
        canister_id,
        settings: CanisterSettings {
            controllers: Some(controllers),
            ..Default::default()
        },
    };

    update_settings(arg)
        .await
        .map_err(|_| Error::Unknown)
        .map(|_| true)
}
