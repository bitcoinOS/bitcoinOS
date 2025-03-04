use candid::Principal;
use wallet::{
    domain::{
        staking::{StakingRecord, StakingStatus},
        TxId,
    },
    utils::ic_time,
};

use super::{p2wsh_multisig22_address, utxos};
use crate::{context::STATE, domain::Metadata, repositories};
use wallet::error::StakingError;

/// TODO: FIXME Should use to HttpOutcalls to query the give txid is confirmed or not
pub(super) async fn serve(
    caller: Principal,
    txid: TxId,
    metadata: Metadata,
) -> Result<Option<StakingRecord>, StakingError> {
    let record = repositories::staking_record::get_staking(txid.clone());
    let network = metadata.network;
    let address = p2wsh_multisig22_address::serve(metadata).await?;
    let utxos = utxos::serve(address, network, None).await?.utxos;
    let mut utxo_total: u64 = 0;
    for utxo in utxos.iter() {
        let utxo_txid = utxo.outpoint.txid.clone();
        let actual_amount = utxo.value;
        if utxo_txid == txid.clone() {
            utxo_total = utxo_total + actual_amount;
        }
    }
    match record {
        Some(r) if r.sender == caller || r.sender_canister == caller => {
            let mut res: Option<StakingRecord> = None;
            STATE.with_borrow_mut(|s| {
                // let txid = utxo.outpoint.txid.clone();
                // let actual_amount = utxo.value;

                if let Some(record) = s.staking_records.get(&txid) {
                    // Change `Pending` to `Confirmed`
                    if record.status == StakingStatus::Pending && record.sent_amount >= utxo_total {
                        let new_record = StakingRecord {
                            actual_amount: utxo_total,
                            updated_time: ic_time(),
                            status: StakingStatus::Confirmed,
                            ..record
                        };

                        res = Some(new_record.clone());

                        s.staking_records.insert(txid, new_record);
                    }
                }
            });

            Ok(res)
        }
        _ => Err(StakingError::UnAuthorized(caller.to_string())),
    }
}
