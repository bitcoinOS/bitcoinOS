use candid::Principal;

use crate::{context::STATE, domain::Metadata, error::WalletError};

use super::validate_controller_mut;

pub(super) fn serve(
    caller: &Principal,
    new_key: String,
    old_key: String,
    updated_time: u64,
) -> Result<bool, WalletError> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();

        validate_controller_mut(&mut state, caller, |s| {
            let metadata = &mut s.metadata;

            let md = metadata.get();
            let current_key = &md.key_name;
            if current_key != &old_key {
                return Err(WalletError::UnAuthorized("old key invalid".to_string()));
            }

            metadata
                .set(Metadata {
                    network: md.network,
                    steward_canister: md.steward_canister,
                    key_name: new_key.clone(),
                    updated_time,
                })
                .map_err(|_| WalletError::ECDSAKeyUpdateError)?;
            Ok(true)
        })
    })
}
