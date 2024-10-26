use crate::{domain::Metadata, repositories};
use wallet::{domain::request::UpdateStakingPoolStatusRequest, error::StakingError};

pub(crate) fn serve(
    metadata: Metadata,
    req: UpdateStakingPoolStatusRequest,
) -> Result<Metadata, StakingError> {
    let new_metadata = Metadata {
        status: req.status.into(),
        ..metadata
    };

    repositories::metadata::save(new_metadata)
}
