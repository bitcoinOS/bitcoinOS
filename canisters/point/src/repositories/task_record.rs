use ic_cdk_timers::TimerId;

use crate::context::TIMER_IDS;

pub fn get_task_record() -> Vec<TimerId> {
    TIMER_IDS.with(|timer_ids| timer_ids.borrow().clone())
}

pub fn new_task_record(timer_id: TimerId) {
    let _ = TIMER_IDS.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

pub fn stop_task() {
    TIMER_IDS.with(|timer_ids| {
        if let Some(timer_id) = timer_ids.borrow_mut().pop() {
            log!(format!("Timer canister: Stopping timer ID {timer_id:?}..."));
            // It's safe to clear non-existent timer IDs.
            ic_cdk_timers::clear_timer(timer_id);
        }
    });
}
