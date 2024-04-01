use candid::Principal;

use crate::{context::STATE, domain::Metadata, error::WalletError};

use super::validate_controller_mut;

pub(super) fn serve(
    caller: &Principal,
    key: String,
    updated_time: u64,
) -> Result<bool, WalletError> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();

        validate_controller_mut(&mut state, caller, |s| {
            let metadata = &mut s.metadata;

            if metadata.get().key_name.is_empty() {
                let md = metadata.get();
                metadata
                    .set(Metadata {
                        network: md.network,
                        steward_canister: md.steward_canister,
                        key_name: key.clone(),
                        updated_time,
                    })
                    .map_err(|_| WalletError::RegisterECDSAKeyError)?;
                Ok(true)
            } else {
                Err(WalletError::ECDSAKeyAlreadyExists(ic_cdk::id().to_string()))
            }
        })
    })
}
