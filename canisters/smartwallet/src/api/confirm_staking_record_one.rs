use ic_cdk::api::management_canister::main::CanisterId;
use wallet::domain::{staking::StakingRecord, TxId};

use crate::{error::WalletError, repositories};

use super::sync_staking_record_status::update_staking_record;

pub(super) async fn serve(txid: TxId) -> Result<Option<StakingRecord>, WalletError> {
    let record = repositories::staking_record::get_staking(&txid);

    match record {
        Some(r) => confirm_and_update_staking_pool_record_one(r.staking_canister, txid).await,
        None => Err(WalletError::StakingRecordNotFound(txid)),
    }
}

async fn confirm_and_update_staking_pool_record_one(
    staking_canister: CanisterId,
    txid: TxId,
) -> Result<Option<StakingRecord>, WalletError> {
    let sync_res = confirm_staking_pool_record_one(staking_canister, txid.clone()).await;

    match sync_res {
        Ok(Some(pool_record)) => {
            update_staking_record(pool_record.clone())?;
            Ok(Some(pool_record))
        }
        Ok(None) => {
            ic_cdk::print(format!("Staking record {txid:?} not found"));
            Ok(None)
        }
        e => e,
    }
}

pub(crate) async fn confirm_staking_pool_record_one(
    stakgin_canister: CanisterId,
    txid: TxId,
) -> Result<Option<StakingRecord>, WalletError> {
    // call remote canister
    let resp: Result<(Option<StakingRecord>,), _> =
        ic_cdk::call(stakgin_canister, "confirm_staking_record_one", (txid,)).await;
    resp.map(|(r,)| r)
        .map_err(|e| WalletError::SyncStakingRecordError(format!("{e:?}")))
}
