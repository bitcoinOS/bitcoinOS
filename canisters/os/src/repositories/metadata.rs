use ic_cdk::api::management_canister::main::CanisterId;

use crate::{context::STATE, domain::Metadata, error::Error};

pub(crate) fn get_metadata() -> Metadata {
    STATE.with(|s| s.borrow().metadata.get().clone())
}

pub(crate) fn set_wallet_cycles(cycle_value: u64) -> Result<u64, Error> {
    STATE.with_borrow_mut(|s| {
        s.metadata
            .set(Metadata {
                wallet_cycles: cycle_value,
                ..s.metadata.get().clone()
            })
            .map_err(|e| Error::StableSetError {
                msg: format!("{e:?}"),
            })
    })?;
    Ok(cycle_value)
}

pub(crate) fn set_steward_canister(canister_id: CanisterId) -> Result<String, Error> {
    STATE.with_borrow_mut(|s| {
        s.metadata
            .set(Metadata {
                steward_canister: canister_id,
                ..s.metadata.get().clone()
            })
            .map_err(|e| Error::StableSetError {
                msg: format!("{e:?}"),
            })
    })?;

    Ok(canister_id.to_string())
}
