use ic_cdk::api::management_canister::main::CanisterId;
use crate::{context::STATE, error::Error};
use crate::repositories::point_record::get_point_records;
use crate::domain::PointRecord;

pub(super) async fn serve() -> Result<Vec<PointRecord>, Error> {
   Ok( get_point_records())
}
