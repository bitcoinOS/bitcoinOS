use ic_cdk::api::management_canister::main::CanisterId;
use wallet::domain::{staking::StakingRecord, TxId};

use crate::{error::WalletError, repositories};

pub(super) async fn serve(txid: TxId) -> Result<(), WalletError> {
    let record = repositories::staking_record::get_staking(&txid);

    match record {
        Some(r) => {
            sync_and_update_staking_record(r.staking_canister, txid).await;
            Ok(())
        }
        None => Err(WalletError::StakingRecordNotFound(txid)),
    }
}

pub(crate) async fn sync_and_update_staking_record(staking_canister: CanisterId, txid: TxId) {
    let sync_res = sync_staking_status(staking_canister, txid.clone())
        .await
        .expect("Failed to sync staking record");

    match sync_res {
        None => ic_cdk::print(format!("Staking record {txid:?} not found")),
        Some(pool_record) => {
            update_staking_record(pool_record).expect("Failed to update staking record")
        }
    }
}

/// Sync staking record from staking pool canister
pub(crate) async fn sync_staking_status(
    stakgin_canister: CanisterId,
    txid: TxId,
) -> Result<Option<StakingRecord>, WalletError> {
    // call remote canister
    let resp: Result<(Option<StakingRecord>,), _> =
        ic_cdk::call(stakgin_canister, "get_staking", (txid,)).await;
    resp.map(|(r,)| r)
        .map_err(|e| WalletError::SyncStakingRecordError(format!("{e:?}")))
}

pub(super) fn update_staking_record(pool_record: StakingRecord) -> Result<(), WalletError> {
    repositories::staking_record::update(pool_record)
}
