use crate::{context::STATE, error::Error};

pub(crate) fn get_counter() -> u128 {
    STATE.with(|s| s.borrow().wallet_counter.get().to_owned())
}

/// Increment the wallet counter by one, and Returns the old value if success
pub(crate) fn increment_one() -> Result<u128, Error> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let current_counter = *state.wallet_counter.get();
        state
            .wallet_counter
            .set(current_counter + 1)
            .map_err(|e| Error::StableSetError {
                msg: format!("{e:?}"),
            })
    })
}
