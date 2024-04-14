use crate::{context::METADATA, error::StewardError};

pub(crate) fn serve() -> Result<String, StewardError> {
    METADATA.with(|m| Ok(m.borrow().get().ecdsa_key_id.name.clone()))
}
