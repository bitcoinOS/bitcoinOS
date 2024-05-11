use wallet::{
    domain::staking::{StakingRecord, StakingStatus},
    utils::ic_time,
};

use crate::{context::STATE, domain::Metadata, error::StakingError};

use super::{p2pkh_address, utxos};

pub(super) async fn serve(metadata: Metadata) -> Result<(), StakingError> {
    let network = metadata.network;
    let address = p2pkh_address::serve(metadata).await?;
    let utxos = utxos::serve(address, network, None).await?.utxos;

    for utxo in utxos.iter() {
        STATE.with_borrow_mut(|s| {
            let txid = utxo.outpoint.txid.clone();
            let actual_amount = utxo.value;

            if let Some(record) = s.staking_records.get(&txid) {
                // Change `Pending` to `Confirmed`
                if record.status == StakingStatus::Pending && record.sent_amount >= utxo.value {
                    let new_record = StakingRecord {
                        actual_amount,
                        updated_time: ic_time(),
                        status: StakingStatus::Confirmed,
                        ..record
                    };

                    s.staking_records.insert(txid, new_record);
                }
            }
        })
    }

    Ok(())
}
