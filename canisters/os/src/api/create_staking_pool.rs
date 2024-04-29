use ic_cdk::api::management_canister::main::CanisterId;

use crate::{
    domain::{request::InitStakingPoolArgument, Metadata},
    error::Error,
};

pub(crate) async fn serve(
    metadata: Metadata,
    os_id: CanisterId,
    created_at: u64,
    arg: InitStakingPoolArgument,
) -> Result<CanisterId, Error> {
    todo!()
}
