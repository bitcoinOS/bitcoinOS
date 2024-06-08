pub mod get_point;
pub mod period_task;
mod save_points;
use crate::{domain::PointRecord, repositories};
use ic_cdk::{
    api::{is_controller, management_canister::main::CanisterId},init,export_candid,
};
use crate::{
    context::STATE,

    domain::{Metadata,request::InitPointArgument},
    constants::{DEFAULT_TIME_PER_PERIOD,DEFAULT_POINT_PER_SAT,POINT_DECIMAL}
};
use std::time::Duration;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);
// static COUNTER: AtomicU64 = AtomicU64::new(0);
#[ic_cdk::query]
fn counter() -> u64 {
    COUNTER.load(Ordering::Relaxed)
}



#[init]
async fn  init(args: InitPointArgument) {
    ic_cdk::print(format!("init point:{args:?}"));
    STATE.with(|s|{
        let state = &mut s.borrow_mut();
        state
            .metadata
            .set(Metadata {
                network: args.network,
                steward_canister: args.steward_canister,
                os_canister: args.os_canister,
                period:DEFAULT_TIME_PER_PERIOD,
                point_per_sat:DEFAULT_POINT_PER_SAT,
                point_decimal:POINT_DECIMAL,
                updated_time:0
            })
            .expect("Failed to init metadata of os canister");
    });
    ic_cdk_timers::set_timer_interval(Duration::from_secs(args.task_period),|| { 
        COUNTER.fetch_add(1, Ordering::Relaxed);
        ic_cdk::spawn(save_points::serve());
    });
    // ic_cdk_timers::set_timer_interval(Duration::from_secs(3), || {
    //     COUNTER.fetch_add(1, Ordering::Relaxed);
    // });
}

#[ic_cdk::query]
fn get_point()-> Vec<PointRecord>{
    get_point::serve()
}

#[ic_cdk::query]
fn get_metadata()-> Metadata{
    repositories::metadata::get_metadata()
}

#[ic_cdk::post_upgrade]
fn post_upgrade(args: InitPointArgument) {
    init(args);
}

export_candid!();