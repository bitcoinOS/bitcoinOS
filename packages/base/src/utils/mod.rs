use std::future::Future;
use std::str::FromStr;

use bitcoin::absolute::LockTime;
use bitcoin::hashes::Hash;
use bitcoin::secp256k1::PublicKey;
use bitcoin::{
    consensus, sighash, Address, Amount, Network, OutPoint, ScriptBuf, SegwitV0Sighash, Sequence,
    Transaction, TxIn, TxOut, Txid, Witness,
};
use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use candid::Principal;
use ic_cdk::api::call::{call_with_payment, CallResult};
use ic_cdk::api::management_canister::bitcoin::{
    BitcoinNetwork, GetBalanceRequest, MillisatoshiPerByte, Satoshi, SendTransactionRequest, Utxo,
};

use crate::constants::{
    DEFAULT_FEE_MILLI_SATOSHI, DUST_AMOUNT_SATOSHI, SEND_TRANSACTION_BASE_CYCLES,
    SEND_TRANSACTION_PER_BYTE_CYCLES, SIG_HASH_TYPE,
};
use crate::domain::{MultiSigIndex, Wallet};
use crate::tx::TransactionInfo;
use crate::{bitcoins, ecdsa, ICBitcoinNetwork};
use crate::{constants::GET_BALANCE_COST_CYCLES, error::Error};

pub type BaseResult<T> = Result<T, Error>;

/// Returns the balance of the given bitcoin address from IC management canister
///
/// NOTE: Relies on the `bitcoin_get_balance` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
pub async fn balance(
    address: impl Into<String>,
    network: BitcoinNetwork,
) -> Result<Satoshi, Error> {
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

/// Build an unsigned transaction for the given amount, network, address
/// Auto choose the utxos to spend
pub async fn build_unsigned_transaction_auto(
    wallet: Wallet,
    amounts: &[Satoshi],
    receiver_addresses: &[&str],
    network: ICBitcoinNetwork,
) -> BaseResult<TransactionInfo> {
    if amounts.len() != receiver_addresses.len() {
        return Err(Error::AmountsAndAddressesMismatch);
    }

    if amounts.iter().any(|amount| *amount < DUST_AMOUNT_SATOSHI) {
        return Err(Error::AmountLessThanDust);
    }

    let fee_percentiles = bitcoins::get_current_fee_percentiles(network).await?;

    let fee_per_byte = if fee_percentiles.is_empty() {
        DEFAULT_FEE_MILLI_SATOSHI
    } else {
        // Choose the median of the percentiles
        fee_percentiles[fee_percentiles.len() / 2]
    };

    ic_cdk::print("Fetching UTXOs... \n");

    // Get the all UTXOS for the given address and network
    let utxos = bitcoins::get_utxos(wallet.address.to_string(), network)
        .await?
        .utxos;

    let receiver_addresses: Result<Vec<Address>, Error> = receiver_addresses
        .iter()
        .map(|address| {
            Address::from_str(address)
                .map_err(|e| Error::BitcoinAddressError(e.to_string()))
                .and_then(|address| {
                    address
                        .require_network(to_bitcoin_network(network))
                        .map_err(|e| e.into())
                })
        })
        .collect();

    let receiver_addresses = receiver_addresses?;

    // Build transaction
    build_transaction_auto(
        &wallet,
        &utxos,
        receiver_addresses.as_slice(),
        amounts,
        fee_per_byte,
    )
    .await
}

/// Build a transaction with the given wallet, amount of `sathoshi`, utxos, receiver_address, fee_per_byte
/// Auto choose the utxos to spend
async fn build_transaction_auto(
    wallet: &Wallet,
    utxos: &[Utxo],
    receiver_addresses: &[Address],
    amounts: &[Satoshi],
    fee_per_byte: MillisatoshiPerByte,
) -> BaseResult<TransactionInfo> {
    ic_cdk::print("Building transaction ... \n");

    let mut total_fee = 0;

    loop {
        let transaction_info =
            build_transaction_with_fee_auto(wallet, utxos, receiver_addresses, amounts, total_fee)?;

        // Calc the transaction size and fee is match use fake signing the transaction.
        let signed_transaction = simulate_signatures(&transaction_info)?.tx;
        let signed_tx_bytes_len = consensus::serialize(&signed_transaction).len() as u64;

        if (signed_tx_bytes_len * fee_per_byte) / 1000 >= total_fee {
            ic_cdk::print(format!("Transaction built with fee: {total_fee:?}."));

            return Ok(transaction_info);
        } else {
            total_fee = (signed_tx_bytes_len * fee_per_byte) / 1000
        }
    }
}

/// Build a transaction with the given wallet, amount of `sathoshi`, utxos, receiver_address, fee
/// auto choose utxos to spend
fn build_transaction_with_fee_auto(
    wallet: &Wallet,
    utxos: &[Utxo],
    receiver_addresses: &[Address],
    amounts: &[Satoshi],
    fee: u64,
) -> BaseResult<TransactionInfo> {
    let mut utxos_to_spend = vec![];
    let mut input_amounts = vec![];
    let mut total_spent = 0;
    let total_amount: Satoshi = amounts.iter().sum();

    for utxo in utxos.iter().rev() {
        total_spent += utxo.value;
        utxos_to_spend.push(utxo);
        input_amounts.push(Amount::from_sat(utxo.value));

        // Auto choose the utxos to spend when the amount is enough
        if total_spent >= total_amount + fee {
            break;
        }
    }

    if total_spent < total_amount + fee {
        return Err(Error::InsufficientFunds);
    }

    // build the tx inputs from the utxos.
    let inputs: Result<Vec<TxIn>, Error> = utxos_to_spend
        .into_iter()
        .map(|utxo| {
            Ok(TxIn {
                previous_output: OutPoint {
                    txid: Txid::from_raw_hash(
                        Hash::from_slice(&utxo.outpoint.txid)
                            .map_err(|e| Error::TransactionHashError(e.to_string()))?,
                    ),
                    vout: utxo.outpoint.vout,
                },
                sequence: Sequence::MAX,
                witness: Witness::new(),
                script_sig: ScriptBuf::new(),
            })
        })
        .collect();

    // build the tx outputs
    let mut outputs: Vec<TxOut> = receiver_addresses
        .iter()
        .zip(amounts)
        .map(|(address, amount)| TxOut {
            script_pubkey: address.script_pubkey(),
            value: Amount::from_sat(*amount),
        })
        .collect();

    let remaining_amount = total_spent - total_amount - fee;

    if remaining_amount >= DUST_AMOUNT_SATOSHI {
        outputs.push(TxOut {
            script_pubkey: wallet.address.script_pubkey(),
            value: Amount::from_sat(remaining_amount),
        })
    }

    let transaction = Transaction {
        input: inputs?,
        output: outputs,
        lock_time: LockTime::ZERO,
        version: bitcoin::blockdata::transaction::Version::ONE,
    };

    let sig_hashes =
        build_transaction_sighashes(&transaction, &wallet.witness_script, input_amounts.clone());

    TransactionInfo::new(transaction, wallet.witness_script.clone(), sig_hashes?)
}

/// Simulatethe signatures for the given tx
fn simulate_signatures(tx_info: &TransactionInfo) -> BaseResult<TransactionInfo> {
    let mut tx = tx_info.tx.clone();

    for input in tx.input.iter_mut() {
        // Clear exists witness
        input.witness.clear();

        let sign = vec![255; 64];

        // Convert signature to DER format
        let mut der_sign = sign_to_der(sign);
        der_sign.push(SIG_HASH_TYPE.to_u32() as u8);

        // Update the signature to the witness
        input.witness.push(vec![]); // Placeholder for scriptSig
        input.witness.push(der_sign.clone());
        input.witness.push(der_sign);
        input
            .witness
            .push(tx_info.witness_script.clone().into_bytes());
    }

    TransactionInfo::new(
        tx,
        tx_info.witness_script.clone(),
        tx_info.sig_hashes.clone(),
    )
}

/// Compute the sighashes for the given transaction
fn build_transaction_sighashes(
    transaction: &Transaction,
    witness_script: &ScriptBuf,
    amounts: Vec<Amount>,
) -> BaseResult<Vec<SegwitV0Sighash>> {
    if transaction.input.len() != amounts.len() {
        return Err(Error::AmountsAndAddressesMismatch);
    }

    let mut cache = sighash::SighashCache::new(transaction);

    transaction
        .input
        .iter()
        .enumerate()
        .zip(amounts)
        .map(|((input_idx, _), amount)| {
            cache
                .p2wsh_signature_hash(
                    input_idx,
                    witness_script,
                    amount,
                    bitcoin::EcdsaSighashType::All,
                )
                .map_err(|e| Error::P2wshSigHashError(e.to_string()))
        })
        .collect()
}

/// Sends a transaction to bitcoin network
///
/// NOTE: Relies on the `bitcoin_send_transaction` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_send_transaction
pub async fn send_transaction(transaction: Vec<u8>, network: BitcoinNetwork) -> BaseResult<()> {
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

/// Create wallet for a given Principal, steward_canister, bitcoin network and key_name
pub async fn create_wallet(
    principal: Principal,
    steward_canister: Principal,
    bitcoin_network: ICBitcoinNetwork,
    key_name: String,
) -> BaseResult<Wallet> {
    check_normal_principal(principal)?;

    // Create a new wallet for this principal.
    // Right now there is only one wallet for each principal,
    // so the it is derived from the principal itself.
    let derivation_path = vec![principal.as_slice().to_vec()];

    // First public key is from the Wallet canister(this canister).
    let pk1 = ecdsa::public_key(key_name, derivation_path.clone(), None).await?;

    // Second public key is generated by the Steward canister.
    let pk2: Result<(Vec<u8>,), Error> = ic_cdk::call(
        steward_canister,
        "public_key",
        (derivation_path.clone(), ),
    )
    .await
    .map_err(|e| e.into());

    let pk2 = pk2?.0;

    let witness_script = bitcoin::blockdata::script::Builder::new()
        .push_int(2)
        .push_slice(PublicKey::from_slice(&pk1)?.serialize())
        .push_slice(PublicKey::from_slice(&pk2)?.serialize())
        .push_int(2)
        .push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKMULTISIG)
        .into_script();

    let script_pub_key = ScriptBuf::new_p2wsh(&witness_script.wscript_hash());

    // Generate the wallet address from the P2WSH script pubkey
    let address =
        bitcoin::Address::from_script(&script_pub_key, to_bitcoin_network(bitcoin_network))
            .map_err(Error::from)?;

    Ok(Wallet {
        witness_script,
        derivation_path,
        address,
    })
}

/// Signature a transaction with given key and derivation path
/// Warning: this functions assumes that the sender is the P2WSH address.
pub async fn sign_transaction(
    tx_info: TransactionInfo,
    key_name: &str,
    derivation_path: &[Vec<u8>],
    signature_index: MultiSigIndex,
) -> BaseResult<TransactionInfo> {
    let (mut tx, sig_hashes) = (tx_info.tx, tx_info.sig_hashes.clone());

    for (tx_in, sighash) in tx.input.iter_mut().zip(sig_hashes) {
        // Clear the witness script if the index is first signature index
        if signature_index == MultiSigIndex::First {
            tx_in.witness.clear();
            tx_in.witness.push(vec![]);
        }

        let sign = ecdsa::sign_with_ecdsa(
            key_name,
            derivation_path.to_owned(),
            sighash.to_byte_array().to_vec(),
        )
        .await;

        // Convert signature to DER format
        let mut der_signature = sign_to_der(sign?);
        der_signature.push(SIG_HASH_TYPE.to_u32() as u8);

        tx_in.witness.push(der_signature);

        // Add witness script if the index is second
        if signature_index == MultiSigIndex::Second {
            tx_in
                .witness
                .push(tx_info.witness_script.clone().into_bytes());
        }
    }

    TransactionInfo::new(tx, tx_info.witness_script.clone(), tx_info.sig_hashes)
}

/// Check a principal is a normal principal or not
/// Returns an error if the principal is not a normal principal
pub fn check_normal_principal(principal: Principal) -> Result<(), Error> {
    if principal != Principal::management_canister() && Principal::anonymous() != principal {
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
    call_with_payment(Principal::management_canister(), method, args, fee)
}

/// Utility function to translate the network string to the IC BitcoinNetwork
pub fn to_ic_bitcoin_network(network: &str) -> ICBitcoinNetwork {
    if network == "mainnet" {
        ICBitcoinNetwork::Mainnet
    } else if network == "testnet" {
        ICBitcoinNetwork::Testnet
    } else {
        ICBitcoinNetwork::Regtest
    }
}

/// Utility function to translate the bitcoin network from the IC cdk
/// to the bitoin network of the rust-bitcoin library.
fn to_bitcoin_network(bitcoin_network: BitcoinNetwork) -> Network {
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
fn sign_to_der(sign: Vec<u8>) -> Vec<u8> {
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
