use ic_stable_structures::cell::ValueError;

use crate::{context::STATE, domain::TimerSettings};

pub fn get() -> TimerSettings {
    STATE.with_borrow(|s| s.timer_settings.get().to_owned())
}

pub fn save(settings: TimerSettings) -> Result<TimerSettings, ValueError> {
    STATE.with_borrow_mut(|s| s.timer_settings.set(settings))
}
