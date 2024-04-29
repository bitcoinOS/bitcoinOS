use crate::{context::STATE, error::Error};

pub(crate) fn get_counter() -> u128 {
    STATE.with(|s| s.borrow().staking_pool_counter.get().to_owned())
}

/// Increment the staking pool counter by one, and Returns the old staking pool counter if success
pub(crate) fn counter_increment_one() -> Result<u128, Error> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let current_counter = *state.staking_pool_counter.get();
        state
            .staking_pool_counter
            .set(current_counter + 1)
            .map_err(|e| Error::StableSetError {
                msg: format!("{e:?}"),
            })
    })
}
