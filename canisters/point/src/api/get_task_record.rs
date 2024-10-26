// use std::borrow::Borrow;

use crate::repositories;

use ic_cdk_timers::TimerId;
pub fn serve() -> Vec<TimerId> {
    let t: TimerId = TimerId::default();
    repositories::task_record::get_task_record()
}
