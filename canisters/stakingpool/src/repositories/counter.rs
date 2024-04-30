use crate::context::STATE;

pub fn get_counter() -> u128 {
    STATE.with(|s| *s.borrow().redeem_counter.get())
}

pub(crate) fn increment_one() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let current_counter = *state.redeem_counter.get();
        let _ = state.redeem_counter.set(current_counter + 1);
    })
}
