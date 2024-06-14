use ic_cdk::api::management_canister::main::CanisterId;

use crate::{context::STATE, domain::Metadata, error::WalletError};

pub(crate) fn get_metadata() -> Metadata {
    STATE.with(|s| s.borrow().metadata.get().clone())
}

pub(crate) fn set_steward_canister(canister_id: CanisterId) -> Result<String, WalletError> {
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
