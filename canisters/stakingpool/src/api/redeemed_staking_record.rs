use std::collections::BTreeSet;

use wallet::{
    domain::staking::{StakingRecord, StakingStatus},
    utils::ic_time,
};

use crate::{context::STATE, domain::Metadata, error::StakingError, repositories};

use super::{p2pkh_address, utxos};

pub(super) async fn serve(metadata: Metadata) -> Result<(), StakingError> {
    let network = metadata.network;
    let address = p2pkh_address::serve(metadata).await?;
    let utxos = utxos::serve(address, network, None).await?.utxos;

    // TODO: FIXME performance issue, utxo maybe a large set
    let utxo_txids: BTreeSet<String> = utxos.iter().map(|u| u.outpoint.txid.clone()).collect();

    let record_txids = repositories::staking_record::keys();

    let redeemed_txids = record_txids.difference(&utxo_txids);

    for txid in redeemed_txids.into_iter() {
        STATE.with_borrow_mut(|s| {
            if let Some(record) = s.staking_records.get(txid) {
                // Change `Redeeming` to `Redeemed`
                if record.status == StakingStatus::Redeeming {
                    let new_record = StakingRecord {
                        updated_time: ic_time(),
                        status: StakingStatus::Redeemed,
                        ..record
                    };

                    s.staking_records.insert(txid.clone(), new_record);
                }
            }
        })
    }

    Ok(())
}
