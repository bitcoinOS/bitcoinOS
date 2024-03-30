use candid::Principal;

use crate::{context::ECDSA_KEYS, domain::ECDSAKey, error::StewardError};

pub(super) fn serve(
    wallet: Principal,
    key: String,
    updated_time: u64,
) -> Result<bool, StewardError> {
    ECDSA_KEYS.with(|keys| {
        let mut ks = keys.borrow_mut();
        match ks.get(&wallet) {
            Some(_) => Err(StewardError::ECDSAKeyAlreadyExists(wallet.to_string())),
            None => {
                ks.insert(wallet, ECDSAKey { key, updated_time });
                Ok(true)
            }
        }
    })
}
