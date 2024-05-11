use ic_cdk::api::management_canister::main::CanisterId;
use wallet::{
    domain::staking::{StakingRecord, StakingStatus},
    utils::ic_time,
};

use crate::{
    domain::{
        request::{StakingRequest, TransferInfo, TransferRequest},
        Metadata,
    },
    error::WalletError,
    repositories,
};

use super::transfer_from_p2pkh;

pub(super) async fn serve(
    public_key: &[u8],
    metadata: Metadata,
    sender_canister: CanisterId,
    sender_address: String,
    sent_time: u64,
    req: StakingRequest,
) -> Result<String, WalletError> {
    let tx_req = TransferRequest {
        txs: vec![TransferInfo {
            recipient: req.staking_address.clone(),
            amount: req.amount,
        }],
    };

    let network = metadata.network;
    let sender = metadata.owner;

    let txid = transfer_from_p2pkh::serve(public_key, metadata, tx_req).await?;

    // Save Staking record in wallet
    let stakings = StakingRecord {
        txid: txid.clone(),
        sender,
        sender_canister,
        sender_address,
        sent_amount: req.amount,
        sent_time,
        annual_interest_rate: 0,
        duration_in_day: 0,
        network,
        staking_canister: req.staking_canister,
        staking_address: req.staking_address,
        actual_amount: 0,
        status: StakingStatus::Pending,
        updated_time: ic_time(),
    };

    repositories::staking_record::save(stakings)?;

    Ok(txid)
}
