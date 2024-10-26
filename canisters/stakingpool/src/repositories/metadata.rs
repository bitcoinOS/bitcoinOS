use ic_cdk::api::management_canister::main::CanisterId;

use crate::{context::STATE, domain::Metadata};
use wallet::error::StakingError;

pub(crate) fn get_metadata() -> Metadata {
    STATE.with(|s| s.borrow().metadata.get().clone())
}

pub(crate) fn save(metadata: Metadata) -> Result<Metadata, StakingError> {
    STATE.with_borrow_mut(|s| {
        let _: Result<Metadata, StakingError> = s.metadata.set(metadata.clone()).map_err(|e| {
            wallet::error::Error::StableWriteError {
                msg: format!("{e:?}"),
            }
            .into()
        });
    });

    Ok(metadata)
}

pub(crate) fn set_steward_canister(canister_id: CanisterId) -> Result<String, StakingError> {
    STATE.with_borrow_mut(|s| {
        s.metadata
            .set(Metadata {
                steward_canister: canister_id,
                ..s.metadata.get().clone()
            })
            .map_err(|e| wallet::error::Error::StableWriteError {
                msg: format!("{e:?}"),
            })
    })?;

    Ok(canister_id.to_string())
}
