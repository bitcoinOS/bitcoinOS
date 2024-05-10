use crate::{domain::TxId, error::WalletError, repositories};

use super::sync_and_update_staking_record;

pub(super) async fn serve(txid: TxId) -> Result<(), WalletError> {
    let record = repositories::staking_record::get_staking(&txid);

    match record {
        Some(r) => {
            sync_and_update_staking_record(r.staking_canister, r.txid).await;
            Ok(())
        }
        None => Err(WalletError::StakingRecordNotFound(txid)),
    }
}
