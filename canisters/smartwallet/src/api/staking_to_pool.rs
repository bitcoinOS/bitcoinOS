use crate::{
    domain::{
        request::{StakingRequest, TransferInfo, TransferRequest},
        Metadata,
    },
    error::WalletError,
};

use super::transfer_from_p2pkh;

pub(super) async fn serve(metadata: Metadata, req: StakingRequest) -> Result<String, WalletError> {
    let tx_req = TransferRequest {
        txs: vec![TransferInfo {
            recipient: req.staking_address,
            amount: req.amount,
        }],
    };

    let txid = transfer_from_p2pkh::serve(metadata, tx_req).await?;

    // Save Staking record in wallet

    Ok(txid)
}
