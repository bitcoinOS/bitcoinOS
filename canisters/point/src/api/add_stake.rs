use candid::{CandidType, Principal};
use ic_cdk::{query, update};
use serde::Deserialize;
use std::cell::Cell;
pub fn add_stake(){

}

#[update]
async fn setup_subscribe(publisher_id: Principal, topic: String) {
    let subscriber = Subscriber { topic };
    let _call_result: Result<(), _> =
        ic_cdk::call(publisher_id, "subscribe", (subscriber,)).await;
}

#[update]
fn add_stake(counter: Counter) {
    COUNTER.with(|c| {
        c.set(c.get() + counter.value);
    });
}