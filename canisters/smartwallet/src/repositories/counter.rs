use crate::context::STATE;

pub fn get_counter() -> u128 {
    STATE.with_borrow(|s| *s.counter.get())
}

pub(crate) fn increment_one() {
    STATE.with_borrow_mut(|s| {
        let current_counter = s.counter.get();
        let _ = s.counter.set(current_counter + 1);
    })
}
