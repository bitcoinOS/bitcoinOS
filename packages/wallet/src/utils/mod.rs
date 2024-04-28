use std::future::Future;
use std::str::FromStr;

use bitcoin::secp256k1::PublicKey;
use bitcoin::{Address, Network, ScriptBuf, SegwitV0Sighash, Transaction};
use hex::ToHex;

use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use candid::Principal;
use ic_cdk::api::call::{call_with_payment, CallResult};

use ic_cdk::api::management_canister::bitcoin::{
    bitcoin_get_balance, BitcoinNetwork, GetBalanceRequest, Satoshi,
};

use crate::error::Error;

pub type WalletResult<T> = Result<T, Error>;

/// Returns the balance of the given bitcoin address from IC management canister
///
/// NOTE: Relies on the `bitcoin_get_balance` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
pub async fn balance(address: impl Into<String>, network: BitcoinNetwork) -> WalletResult<Satoshi> {
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

/// Create wallet for a given Principal, pk1, pk2 and bitcoin network
///
pub async fn create_wallet(
    principal: Principal,
    pk1: &[u8],
    pk2: &[u8],
    bitcoin_network: Network,
) -> WalletResult<Address> {
    check_normal_principal(principal).map_err(Error::from)?;

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

/// Check a principal is a normal principal or not
/// Returns an error if the principal is not a normal principal
pub fn check_normal_principal(principal: Principal) -> Result<(), Error> {
    if principal != mgmt_canister_id() && Principal::anonymous() != principal {
        Ok(())
    } else {
        Err(Error::InvalidPrincipal(principal))
    }
}

/// A helper function to call management canister with payment
pub fn call_management_with_payment<T: ArgumentEncoder, R: for<'a> ArgumentDecoder<'a>>(
    method: &str,
    args: T,
    fee: u64,
) -> impl Future<Output = CallResult<R>> + Send + Sync {
    call_with_payment(mgmt_canister_id(), method, args, fee)
}

/// Utility function to translate the network string to the IC BitcoinNetwork
pub fn to_ic_bitcoin_network(network: &str) -> BitcoinNetwork {
    if network == "mainnet" {
        BitcoinNetwork::Mainnet
    } else if network == "testnet" {
        BitcoinNetwork::Testnet
    } else {
        BitcoinNetwork::Regtest
    }
}

/// A helper function to convert a string to a Address of ust-bitcoin library with network
pub fn str_to_bitcoin_address(address: &str, network: BitcoinNetwork) -> Result<Address, Error> {
    Address::from_str(address)
        .map_err(|e| Error::InvalidBitcoinAddress(e.to_string()))
        .and_then(|address| {
            address
                .require_network(to_bitcoin_network(network))
                .map_err(|e| e.into())
        })
}

/// Utility function to translate the bitcoin network from the IC cdk
/// to the bitoin network of the rust-bitcoin library.
pub fn to_bitcoin_network(bitcoin_network: BitcoinNetwork) -> Network {
    match bitcoin_network {
        BitcoinNetwork::Mainnet => Network::Bitcoin,
        BitcoinNetwork::Testnet => Network::Testnet,
        BitcoinNetwork::Regtest => Network::Regtest,
    }
}

/// Check the length of the transaction and the signatures
pub fn check_tx_hashes_len(
    transaction: &Transaction,
    sig_hashes: &[SegwitV0Sighash],
) -> Result<(), Error> {
    if transaction.input.len() != sig_hashes.len() {
        Err(Error::TransactionAndSignaturesMismatch)
    } else {
        Ok(())
    }
}

/// Converts a SEC1 ECDSA signature to the DER format.
pub fn sign_to_der(sign: Vec<u8>) -> Vec<u8> {
    let r: Vec<u8> = if sign[0] & 0x80 != 0 {
        // r is negative. Prepend a zero byte.
        let mut tmp = vec![0x00];
        tmp.extend(sign[..32].to_vec());
        tmp
    } else {
        // r is positive.
        sign[..32].to_vec()
    };

    let s: Vec<u8> = if sign[32] & 0x80 != 0 {
        // s is negative. Prepend a zero byte.
        let mut tmp = vec![0x00];
        tmp.extend(sign[32..].to_vec());
        tmp
    } else {
        // s is positive.
        sign[32..].to_vec()
    };

    // Convert signature to DER.
    vec![
        vec![0x30, 4 + r.len() as u8 + s.len() as u8, 0x02, r.len() as u8],
        r,
        vec![0x02, s.len() as u8],
        s,
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub fn mgmt_canister_id() -> Principal {
    Principal::from_str("aaaaa-aa").unwrap()
}

pub fn principal_to_derivation_path(principal: Principal) -> Vec<Vec<u8>> {
    vec![principal.as_slice().to_vec()]
}

pub fn hex<T: AsRef<[u8]>>(data: T) -> String {
    data.encode_hex()
}

pub fn ic_caller() -> Principal {
    ic_cdk::caller()
}

pub fn ic_time() -> u64 {
    ic_cdk::api::time()
}

pub fn canister_id() -> Principal {
    ic_cdk::id()
}
