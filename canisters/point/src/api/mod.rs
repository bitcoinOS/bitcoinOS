pub mod get_point;
pub mod period_task;
use ic_cdk::{
    api::{is_controller, management_canister::main::CanisterId},init,
};
use crate::{
    context::STATE,

    domain::{Metadata,request::InitPointArgument},
    constants::{DEFAULT_TIME_PER_PERIOD,DEFAULT_POINT_PER_SAT,POINT_DECIMAL}
};

#[init]
fn init(args: InitPointArgument) {

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
    })
}