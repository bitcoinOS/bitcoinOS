use candid::Principal;
use wallet::domain::request::{StakingRequest, TransferInfo, TransferRequest};

use crate::{domain::Metadata, error::DBankError};

use super::transfer_from_p2wpkh;

pub(super) async fn serve(
    public_key: &[u8],
    wallet_owner: Principal,
    metadata: Metadata,
    req: StakingRequest,
) -> Result<String, DBankError> {
    let tx_req = TransferRequest {
        txs: vec![TransferInfo {
            recipient: req.staking_address.clone(),
            amount: req.amount,
        }],
    };

    let network = metadata.network;

    let txs = tx_req.validate_address(network)?;
    let txid = transfer_from_p2wpkh::serve(public_key, wallet_owner, metadata, &txs.txs).await?;

    Ok(txid)
}
