use crate::{domain::Metadata, repositories};
use wallet::{
    domain::{request::UpdateStakingPoolInfoRequest, response::UpdateStakingPoolInfoResponse},
    error::StakingError,
};

pub(crate) fn serve(
    metadata: Metadata,
    req: UpdateStakingPoolInfoRequest,
) -> Result<UpdateStakingPoolInfoResponse, StakingError> {
    let new_metadata = Metadata {
        name: req.name,
        description: req.description,
        annual_interest_rate: req.annual_interest_rate,
        duration_in_day: req.duration_in_day,
        status: req.status.into(),
        start_time: req.start_time,
        end_time: req.end_time,
        ..metadata
    };

    repositories::metadata::save(new_metadata).map(|m| m.into())
}
