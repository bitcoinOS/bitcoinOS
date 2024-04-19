use candid::Principal;

use crate::error::WalletError;

use super::validate_controller;

/// Only controller can call this
pub(crate) fn serve(caller: Principal) -> Result<String, WalletError> {
    validate_controller(caller).map(|m| m.ecdsa_key_id.name.clone())
}
