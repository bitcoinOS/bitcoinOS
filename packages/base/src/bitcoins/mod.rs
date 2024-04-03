use ic_cdk::api::management_canister::bitcoin::{
    GetCurrentFeePercentilesRequest, GetUtxosRequest, GetUtxosResponse, MillisatoshiPerByte,
};

use crate::{
    constants::{GET_CURRENT_FEE_PERCENTILES_CYCLES, GET_UTXOS_COST_CYCLES},
    utils::{call_management_with_payment, BaseResult},
    ICBitcoinNetwork,
};

/// Returns the current fee percentiles measured in millisatoshi per byte
/// Percentiles are computed from the last 10,000 transactions (if available).
///
/// NOTE: Relies on the `bitcoin_get_current_fee_percentiles` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-
pub async fn get_current_fee_percentiles(
    network: ICBitcoinNetwork,
) -> BaseResult<Vec<MillisatoshiPerByte>> {
    let args = (GetCurrentFeePercentilesRequest { network },);
    let fee = GET_CURRENT_FEE_PERCENTILES_CYCLES;

    call_management_with_payment("bitcoin_get_current_fee_percentiles", args, fee)
        .await
        .map(|(percentiles,)| percentiles)
        .map_err(|e| e.into())
}

/// Returns UTXOs of the given bitcoin address
///
/// NOTE: Relies on the `bitcoin_get_utxos` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
pub async fn get_utxos(
    address: impl Into<String>,
    network: ICBitcoinNetwork,
) -> BaseResult<GetUtxosResponse> {
    let args = (GetUtxosRequest {
        address: address.into(),
        network,
        filter: None,
    },);

    let fee = GET_UTXOS_COST_CYCLES;

    call_management_with_payment("bitcion_get_utxos", args, fee)
        .await
        .map(|(utxo,)| utxo)
        .map_err(|e| e.into())
}
