use bitcoin::secp256k1::PublicKey;
use bitcoin::ScriptBuf;

use candid::Principal;
use ic_cdk::api::management_canister::{
    bitcoin::{
        bitcoin_get_balance, bitcoin_get_current_fee_percentiles, bitcoin_get_utxos,
        bitcoin_send_transaction, BitcoinNetwork, GetBalanceRequest,
        GetCurrentFeePercentilesRequest, GetUtxosRequest, GetUtxosResponse, MillisatoshiPerByte,
        Satoshi, SendTransactionRequest, UtxoFilter,
    },
    ecdsa::EcdsaKeyId,
};
use sha2::Digest;

use crate::{
    domain::{Wallet, WalletType},
    ecdsa,
    utils::{to_bitcoin_network, BaseResult},
};

/// Returns the balance of the given bitcoin address from IC management canister
///
/// NOTE: Relies on the `bitcoin_get_balance` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
pub async fn balance(address: impl Into<String>, network: BitcoinNetwork) -> BaseResult<Satoshi> {
    let arg = GetBalanceRequest {
        address: address.into(),
        network,
        min_confirmations: None,
    };

    bitcoin_get_balance(arg)
        .await
        .map(|(balance,)| balance)
        .map_err(|e| e.into())
}

/// Returns the current fee percentiles measured in millisatoshi per byte
/// Percentiles are computed from the last 10,000 transactions (if available).
///
/// NOTE: Relies on the `bitcoin_get_current_fee_percentiles` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-
pub async fn get_current_fee_percentiles(
    network: BitcoinNetwork,
) -> BaseResult<Vec<MillisatoshiPerByte>> {
    let arg = GetCurrentFeePercentilesRequest { network };

    bitcoin_get_current_fee_percentiles(arg)
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
    network: BitcoinNetwork,
    filter: Option<UtxoFilter>,
) -> BaseResult<GetUtxosResponse> {
    let arg = GetUtxosRequest {
        address: address.into(),
        network,
        filter,
    };

    bitcoin_get_utxos(arg)
        .await
        .map(|(utxo,)| utxo)
        .map_err(|e| e.into())
}

/// Sends a transaction to bitcoin network
///
/// NOTE: Relies on the `bitcoin_send_transaction` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_send_transaction
pub async fn send_transaction(transaction: Vec<u8>, network: BitcoinNetwork) -> BaseResult<()> {
    let arg = SendTransactionRequest {
        transaction,
        network,
    };

    bitcoin_send_transaction(arg).await.map_err(|e| e.into())
}

/// Returns the P2PKH address of this canister at the given derivation path.
pub async fn get_p2pkh_address(
    network: BitcoinNetwork,
    derivation_path: Vec<Vec<u8>>,
    key_id: EcdsaKeyId,
    canister_id: Option<Principal>,
) -> BaseResult<String> {
    let public_key = ecdsa::public_key(derivation_path, key_id, canister_id).await;

    public_key.map(|key| public_key_to_p2pkh_address(network, &key))
}

/// Convert public key to P2PKH address
pub fn public_key_to_p2pkh_address(network: BitcoinNetwork, public_key: &[u8]) -> String {
    // SHA-256 & RIPEMD-160
    let res = ripemd160(&sha256(public_key));

    let prefix = match network {
        BitcoinNetwork::Mainnet => 0x00,
        BitcoinNetwork::Testnet | BitcoinNetwork::Regtest => 0x6F,
    };

    let mut data_with_prefix = vec![prefix];
    data_with_prefix.extend(res);

    let checksum = &sha256(&sha256(&data_with_prefix.clone()))[..4];

    let mut full_address = data_with_prefix;
    full_address.extend(checksum);

    bs58::encode(full_address).into_string()
}

/// Create a wallet with p2wsh address from public key
/// Returns Wallet
pub async fn create_p2wsh_wallet(
    derivation_path: Vec<Vec<u8>>,
    public_key: &[u8],
    network: BitcoinNetwork,
) -> BaseResult<Wallet> {
    let public_key = PublicKey::from_slice(public_key)?;

    // let witness_script = ScriptBuf::new_p2pkh(&public_key.pubkey_hash());

    // let script_pub_key = ScriptBuf::new_p2wsh(&witness_script.wscript_hash());
    let witness_script = bitcoin::blockdata::script::Builder::new()
        .push_int(1)
        .push_slice(public_key.serialize())
        .into_script();
    let script_buf = ScriptBuf::new_p2wsh(&witness_script.wscript_hash());

    let address =
        bitcoin::Address::from_script(script_buf.as_script(), to_bitcoin_network(network))?;

    Ok(Wallet {
        witness_script,
        address,
        derivation_path,
        wallet_type: WalletType::Single,
    })
}

fn ripemd160(data: &[u8]) -> Vec<u8> {
    let mut hasher = ripemd::Ripemd160::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
