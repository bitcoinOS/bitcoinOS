
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
                // Change `Redeeming` to `Redeemed`
                if record.status == StakingStatus::Redeeming {
                    let new_record = StakingRecord {
                        actual_amount,
                        updated_time: ic_time(),
                        status: StakingStatus::Redeemed,
                        ..record
                    };

                    s.staking_records.insert(txid, new_record);
                }
            }
        })
    }

    Ok(())
}
