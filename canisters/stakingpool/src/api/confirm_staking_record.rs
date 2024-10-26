use wallet::{
    domain::staking::{StakingRecord, StakingStatus},
    utils::ic_time,
};

use super::{p2wsh_multisig22_address, utxos};
use crate::{context::STATE, domain::Metadata};
use std::collections::HashMap;
use wallet::error::StakingError;

pub(super) async fn serve(metadata: Metadata) -> Result<(), StakingError> {
    let network = metadata.network;
    // let address = p2pkh_address::serve(metadata).await?;
    let address = p2wsh_multisig22_address::serve(metadata).await?;
    // ic_cdk::print(format!("in confirm 1 \n{}", address.clone()));
    let utxos = utxos::serve(address, network, None).await?.utxos;

    let mut tx_utxo_map = HashMap::new();
    // ic_cdk::print(format!("in confirm 2 \n{}", utxos.len()));
    for utxo in utxos.iter() {
        let txid = utxo.outpoint.txid.clone();
        // ic_cdk::print(format!("in confirm 3 \n{}", txid.clone()));
        let actual_amount = utxo.value;
        let old_utxo = tx_utxo_map.get(txid.as_str());
        if let Some(v) = old_utxo {
            tx_utxo_map.insert(txid.clone(), v + actual_amount);
        } else {
            tx_utxo_map.insert(txid.clone(), actual_amount);
        }
    }

    for (tx, utxo) in tx_utxo_map.into_iter() {
        STATE.with_borrow_mut(|s| {
            if let Some(record) = s.staking_records.get(&tx) {
                // Change `Pending` to `Confirmed`
                if record.status == StakingStatus::Pending && record.sent_amount >= utxo {
                    let new_record = StakingRecord {
                        actual_amount: utxo,
                        updated_time: ic_time(),
                        status: StakingStatus::Confirmed,
                        ..record
                    };

                    s.staking_records.insert(tx, new_record);
                }
            }
        })
    }

    Ok(())
}
