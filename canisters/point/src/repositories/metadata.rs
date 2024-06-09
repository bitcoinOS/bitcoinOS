use crate::{context::STATE, domain::Metadata, error::Error};

pub(crate) fn get_metadata() -> Metadata {
    STATE.with(|s| s.borrow().metadata.get().clone())
}

pub(crate) fn set_period(period: u64) -> Result<u64, Error> {
    STATE.with_borrow_mut(|s| {
        s.metadata
            .set(Metadata {
                period,
                ..s.metadata.get().clone()
            })
            .map_err(|e| Error::StableSetError {
                msg: format!("{e:?}"),
            })
    })?;
    Ok(period)
}

pub(crate) fn set_point_per_sat(point_per_sat: u64) -> Result<u64, Error> {
    STATE.with_borrow_mut(|s| {
        s.metadata
            .set(Metadata {
                point_per_sat,
                ..s.metadata.get().clone()
            })
            .map_err(|e| Error::StableSetError {
                msg: format!("{e:?}"),
            })
    })?;
    Ok(point_per_sat)
}
