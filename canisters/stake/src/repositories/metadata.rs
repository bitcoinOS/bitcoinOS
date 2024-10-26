use crate::{context::STATE, domain::Metadata, error::Error};

pub(crate) fn get_metadata() -> Metadata {
    STATE.with(|s| s.borrow().metadata.get().clone())
}
