use std::future::Future;

use bitcoin::secp256k1::PublicKey;
use bitcoin::{Address, Network, ScriptBuf};
use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use candid::Principal;
use ic_cdk::api::call::{call_with_payment, CallResult};
use ic_cdk::api::management_canister::bitcoin::{
    BitcoinNetwork, GetBalanceRequest, GetCurrentFeePercentilesRequest, GetUtxosRequest,
    GetUtxosResponse, MillisatoshiPerByte, Satoshi, SendTransactionRequest,
};

use crate::constants::{
    GET_CURRENT_FEE_PERCENTILES_CYCLES, GET_UTXOS_COST_CYCLES, SEND_TRANSACTION_BASE_CYCLES,
    SEND_TRANSACTION_PER_BYTE_CYCLES,
};
use crate::{constants::GET_BALANCE_COST_CYCLES, error::WalletError};

pub type WalletResult<T> = Result<T, WalletError>;

/// Returns the balance of the given bitcoin address from IC management canister
///
/// NOTE: Relies on the `bitcoin_get_balance` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
pub async fn get_balance(
    address: impl Into<String>,
    network: BitcoinNetwork,
) -> Result<Satoshi, WalletError> {
    let args = (GetBalanceRequest {
        address: address.into(),
        network,
        min_confirmations: None,
    },);

    let fee = GET_BALANCE_COST_CYCLES;

    call_management_with_payment("bitcoin_get_balance", args, fee)
        .await
        .map(|(balance,)| balance)
        .map_err(|e| e.into())
}

/// Returns UTXOs of the given bitcoin address
///
/// NOTE: Relies on the `bitcoin_get_utxos` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
pub async fn get_utxos(
    address: impl Into<String>,
    network: BitcoinNetwork,
) -> Result<GetUtxosResponse, WalletError> {
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

/// Returns the current fee percentiles measured in millisatoshi per byte
/// Percentiles are computed from the last 10,000 transactions (if available).
///
/// NOTE: Relies on the `bitcoin_get_current_fee_percentiles` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-
pub async fn get_current_fee_percentiles(
    network: BitcoinNetwork,
) -> WalletResult<MillisatoshiPerByte> {
    let args = (GetCurrentFeePercentilesRequest { network },);
    let fee = GET_CURRENT_FEE_PERCENTILES_CYCLES;

    call_management_with_payment("bitcoin_get_current_fee_percentiles", args, fee)
        .await
        .map(|(percentiles,)| percentiles)
        .map_err(|e| e.into())
}

/// Sends a transaction to bitcoin network
///
/// NOTE: Relies on the `bitcoin_send_transaction` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_send_transaction
pub async fn send_transaction(transaction: Vec<u8>, network: BitcoinNetwork) -> WalletResult<()> {
    let fee = SEND_TRANSACTION_BASE_CYCLES
        + (transaction.len() as u64) * SEND_TRANSACTION_PER_BYTE_CYCLES;

    let args = (SendTransactionRequest {
        transaction,
        network,
    },);

    call_management_with_payment("bitcoin_send_transaction", args, fee)
        .await
        .map(|((),)| ())
        .map_err(|e| e.into())
}

/// Create wallet for a given Principal, pk1, pk2 and bitcoin network
///
pub async fn create_wallet(
    principal: Principal,
    pk1: &[u8],
    pk2: &[u8],
    bitcoin_network: Network,
) -> WalletResult<Address> {
    if !is_normal_principal(principal) {
        return Err(WalletError::InvalidPrincipal(principal));
    }

    let witness_script = bitcoin::blockdata::script::Builder::new()
        .push_int(2)
        .push_slice(PublicKey::from_slice(pk1)?.serialize())
        .push_slice(PublicKey::from_slice(pk2)?.serialize())
        .push_int(2)
        .push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKMULTISIG)
        .into_script();

    let script_pub_key = ScriptBuf::new_p2wsh(&witness_script.wscript_hash());

    // Generate the wallet address from the P2WSH script pubkey
    bitcoin::Address::from_script(&script_pub_key, bitcoin_network).map_err(|e| e.into())
}

pub fn is_normal_principal(principal: Principal) -> bool {
    principal != Principal::management_canister() && Principal::anonymous() != principal
}

pub fn call_management_with_payment<T: ArgumentEncoder, R: for<'a> ArgumentDecoder<'a>>(
    method: &str,
    args: T,
    fee: u64,
) -> impl Future<Output = CallResult<R>> + Send + Sync {
    call_with_payment(Principal::management_canister(), method, args, fee)
}
