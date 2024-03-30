use candid::Principal;

use crate::{context::ECDSA_KEYS, domain::ECDSAKey, error::StewardError};

pub(super) fn serve(
    wallet: Principal,
    new_key: String,
    old_key: String,
    updated_time: u64,
) -> Result<bool, StewardError> {
    ECDSA_KEYS.with(|keys| {
        let mut ks = keys.borrow_mut();
        match ks.get(&wallet) {
            Some(key) => {
                if key.key == old_key {
                    ks.insert(
                        wallet,
                        ECDSAKey {
                            key: new_key,
                            updated_time,
                        },
                    );
                    Ok(true)
                } else {
                    Err(StewardError::ECDSAKeyUpdateError)
                }
            }
            None => Err(StewardError::ECDSAKeyNotFound(wallet.to_string())),
        }
    })
}
