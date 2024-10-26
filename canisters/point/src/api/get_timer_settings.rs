use crate::{domain::TimerSettings, repositories};

pub fn serve() -> TimerSettings {
    repositories::timer_settings::get()
}
