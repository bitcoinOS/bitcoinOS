use crate::{
    domain::{StakingRecord, TxId},
    repositories::staking_record,
};

pub(super) fn serve(txid: TxId) -> Option<StakingRecord> {
    staking_record::get_staking(txid)
}
