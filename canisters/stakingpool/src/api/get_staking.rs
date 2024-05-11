use crate::repositories::staking_record;

use wallet::domain::staking::{StakingRecord, TxId};
pub(super) fn serve(txid: TxId) -> Option<StakingRecord> {
    staking_record::get_staking(txid)
}
