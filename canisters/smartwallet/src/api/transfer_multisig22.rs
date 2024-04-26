use candid::Principal;

use crate::{
    domain::{request::TransferRequest, Metadata},
    error::WalletError,
};

use super::build_transaction_multisig22;

pub(super) async fn serve(
    owner: Principal,
    metadata: Metadata,
    req: TransferRequest,
) -> Result<String, WalletError> {
    let tx_info = build_transaction_multisig22::serve(owner, metadata.clone(), req)
        .await
        .unwrap();

    let key_id = metadata.ecdsa_key_id;
    let network = metadata.network;
    let steward_caninster = metadata.steward_canister;
    let wallet_canister = ic_cdk::id();

    ic_cdk::print(format!("{:?}", tx_info));

    let resp: Result<(String,), _> = ic_cdk::api::call::call(
        steward_caninster,
        "finalize_tx_and_send",
        (tx_info, key_id, wallet_canister, network),
    )
    .await;

    resp.map(|(txid,)| txid)
        .map_err(|e| WalletError::StewardCallError(format!("{:?}", e)))
}
