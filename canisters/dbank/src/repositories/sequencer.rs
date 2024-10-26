use crate::context::STATE;

pub fn get_sequencer() -> u64 {
    STATE.with_borrow(|s| *s.sequencer.get())
}

pub(crate) fn increment_one() -> u64 {
    STATE.with_borrow_mut(|s| {
        let current_counter = s.sequencer.get();
        let new_sequencer = current_counter + 1;
        let _ = s.sequencer.set(new_sequencer);

        new_sequencer
    })
}
