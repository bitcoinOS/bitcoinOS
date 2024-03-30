use candid::Principal;

use crate::{context::ECDSA_KEYS, error::StewardError};

pub(crate) fn serve(caller: &Principal) -> Result<String, StewardError> {
    ECDSA_KEYS.with(|key| {
        key.borrow()
            .get(caller)
            .map(|k| k.key.clone())
            .ok_or_else(|| StewardError::ECDSAKeyNotFound(caller.to_string()))
    })
}
