use candid::Principal;

use crate::{context::STATE, error::WalletError};

use super::validate_controller;

/// Only controller can call this
pub(crate) fn serve(caller: &Principal) -> Result<String, WalletError> {
    STATE.with(|s| {
        let state = s.borrow();

        validate_controller(&state, caller, |s| {
            let key = &s.metadata.get().ecdsa_key_id;
            Ok(key.name.clone())
        })
    })
}
