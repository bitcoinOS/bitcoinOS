use std::future::Future;
use std::str::FromStr;

use bitcoin::absolute::LockTime;
use bitcoin::hashes::Hash;
use bitcoin::script::{Builder, PushBytesBuf};
use bitcoin::transaction::Version;
use bitcoin::{
    consensus, sighash, Address, AddressType, Amount, EcdsaSighashType, Network, OutPoint, Script,
    SegwitV0Sighash, Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};
use hex::ToHex;

use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use candid::Principal;
use ic_cdk::api::call::{call_with_payment, CallResult};

use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, MillisatoshiPerByte, Satoshi};
use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;

use crate::constants::DEFAULT_FEE_MILLI_SATOSHI;
use crate::domain::response::Utxo;
use crate::domain::Wallet;
use crate::error::Error;
use crate::tx::{RecipientAmount, TransactionInfo};
use crate::{bitcoins, ecdsa};

pub type WalletResult<T> = Result<T, Error>;

/// Create a new P2PKH wallet with given arguments
pub async fn create_p2pkh_wallet(
    owner: Principal,
    key_id: EcdsaKeyId,
    network: BitcoinNetwork,
) -> Result<Wallet, Error> {
    let derivation_path = principal_to_derivation_path(owner);
    let public_key = ecdsa::public_key(derivation_path.clone(), key_id, None).await?;

    bitcoins::create_p2pkh_wallet(derivation_path, &public_key, network).await
}

/// Create a new P2WPKH wallet with given arguments
pub async fn create_p2wpkh_wallet(
    owner: Principal,
    key_id: EcdsaKeyId,
    network: BitcoinNetwork,
) -> Result<Wallet, Error> {
    let derivation_path = principal_to_derivation_path(owner);
    let public_key = ecdsa::public_key(derivation_path.clone(), key_id, None).await?;

    bitcoins::create_p2wpkh_wallet(derivation_path, &public_key, network).await
}

/// Build a transaction using given parameters
/// NOTE: There's a chicken-and-egg problem when compute the fee for a transaction.
/// but we need to know the proper fee in order to figure   out the inputs needed for the transaction
///
/// Solving this problem is iteratively now, we start with a fee of zero, build and sign a transactoin,
/// find what its size is, and then update the fee, rebuild the transaction, until the fee is correct.
pub async fn build_transaction(
    public_key: &[u8],
    sender: &Address,
    utxos: &[Utxo],
    txs: &[RecipientAmount],
    fee_per_byte: MillisatoshiPerByte,
) -> Result<Transaction, Error> {
    ic_cdk::print("Building transaction ... \n");

    let mut total_fee = 0;

    loop {
        let tx = calc_fee_and_build_transaction(sender, utxos, txs, total_fee)?;

        // Sign the transaction with mock value
        let signed_tx = sign_transaction_p2pkh(
            public_key,
            sender,
            tx.clone(),
            EcdsaKeyId::default(),
            vec![],
            mock_signer,
        )
        .await?;

        let signed_tx_bytes_len = consensus::serialize(&signed_tx).len() as u64;

        if (signed_tx_bytes_len * fee_per_byte) / 1000 == total_fee {
            ic_cdk::print(format!("Transaction built with fee: {total_fee:?}."));
            return Ok(tx);
        } else {
            total_fee = (signed_tx_bytes_len * fee_per_byte) / 1000;
        }
    }
}

/// Build a transaction using given parameters
pub async fn build_unsigned_p2wpkh_transaction(
    network: BitcoinNetwork,
    my_wallet: &Wallet,
    txs: &[RecipientAmount],
    sighash_type: EcdsaSighashType,
) -> Result<(TransactionInfo, Vec<Amount>), Error> {
    let fee_per_byte = bitcoins::get_fee_per_byte(network, DEFAULT_FEE_MILLI_SATOSHI).await?;

    // Fetch UTXOs
    ic_cdk::print("Fetching Utxos... \n");

    // FIXME: UTXOs maybe very large, need to paginate
    let utxos = bitcoins::get_utxos(my_wallet.address.to_string(), network, None)
        .await?
        .utxos;

    bitcoins::p2wpkh::build_p2wpkh_transaction_info(
        my_wallet,
        &utxos,
        txs,
        fee_per_byte,
        sighash_type,
    )
    .await
}

/// Calculate the fee for a given arguments, and build the transaction with argument and fee if the amounts in utxos are enough
/// Returns the transaction if success, otherwise return `InsufficientFunds`
pub fn calc_fee_and_build_transaction(
    sender: &Address,
    utxos: &[Utxo],
    txs: &[RecipientAmount],
    fee: Satoshi,
) -> Result<Transaction, Error> {
    // Assume that any amount below this threshold is dust.
    const DUST_THRESHOLD: u64 = 1_000;

    // FIXME Optimize UTXO selection
    // Select which UTXOs to spend. We naively spend the oldest available UTXOs,
    // even if they were previously spent in a transaction. This isn't a
    // problem as long as at most one transaction is created per block and
    // we're using min_confirmations of 1.
    let mut utxos_to_spend = vec![];

    // The total spent amount is included fee
    let mut input_amount_satoshi = 0;
    let amount: u64 = txs.iter().map(|r| r.amount.to_sat()).sum();
    let output_amount_satoshi = amount + fee;

    for utxo in utxos.iter().rev() {
        ic_cdk::print(format!("utxo value is: {:?} -------- \n", utxo.value));
        input_amount_satoshi += utxo.value;
        utxos_to_spend.push(utxo);

        if input_amount_satoshi >= output_amount_satoshi {
            // Inputs amount is enough
            break;
        }
    }

    ic_cdk::print(format!("The input amount in satoshi: {input_amount_satoshi:?}, the output amount in satoshi: {output_amount_satoshi:?}, the fee is {fee:?} -------- \n"));

    // Check input amount in utxos is greater than output amount
    if input_amount_satoshi < output_amount_satoshi {
        return Err(Error::InsufficientFunds);
    }

    let input: Vec<TxIn> = utxos_to_spend
        .into_iter()
        .map(|u| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_str(&u.outpoint.txid).expect("Failed to parse txid"),
                vout: u.outpoint.vout,
            },
            sequence: Sequence::MAX,
            witness: Witness::new(),
            script_sig: Script::new().to_owned(),
        })
        .collect();

    let mut output: Vec<TxOut> = txs
        .iter()
        .map(|r| TxOut {
            script_pubkey: r.recipient.script_pubkey(),
            value: r.amount,
        })
        .collect();

    // Handle the change
    let remaining_amount = input_amount_satoshi - output_amount_satoshi;

    if remaining_amount >= DUST_THRESHOLD {
        output.push(TxOut {
            script_pubkey: sender.script_pubkey(),
            value: Amount::from_sat(remaining_amount),
        });
    }

    Ok(Transaction {
        input,
        output,
        lock_time: LockTime::ZERO,
        version: Version::ONE,
    })
}

/// Sign a transaction with P2PKH address
/// NOTE: Only support P2PKH
pub async fn sign_transaction_p2pkh<SignFun, Fut>(
    public_key: &[u8],
    sender: &Address,
    mut tx: Transaction,
    key_id: EcdsaKeyId,
    derivation_path: Vec<Vec<u8>>,
    signer: SignFun,
) -> Result<Transaction, Error>
where
    SignFun: Fn(Vec<Vec<u8>>, EcdsaKeyId, Vec<u8>) -> Fut,
    Fut: std::future::Future<Output = Vec<u8>>,
{
    // Check if the sender is P2PKH
    validate_p2pkh_address(sender)?;

    let tx_clone = tx.clone();

    // let sig_hashes = vec![];
    let sig_cache = sighash::SighashCache::new(&tx_clone);

    for (idx, tx_in) in tx.input.iter_mut().enumerate() {
        let sighash = sig_cache
            .legacy_signature_hash(idx, &sender.script_pubkey(), EcdsaSighashType::All.to_u32())
            .unwrap();
        let signature = signer(
            derivation_path.clone(),
            key_id.clone(),
            sighash.as_byte_array().to_vec(),
        )
        .await;

        let der_signature = sign_to_der(signature);

        let mut sig_with_hashtype = der_signature;

        sig_with_hashtype.push(EcdsaSighashType::All.to_u32() as u8);

        let sig_push_bytes: PushBytesBuf = sig_with_hashtype.try_into().unwrap();
        let public_key_push_bytes: PushBytesBuf = public_key.to_vec().try_into().unwrap();
        tx_in.script_sig = Builder::new()
            .push_slice(sig_push_bytes.as_ref())
            .push_slice(public_key_push_bytes.as_ref())
            .into_script();

        tx_in.witness.clear();
    }

    Ok(tx)
}

pub fn validate_p2pkh_address(address: &Address) -> Result<(), Error> {
    if address.address_type() == Some(AddressType::P2pkh) {
        Ok(())
    } else {
        Err(Error::OnlySupportP2pkhSign)
    }
}

pub fn validate_p2wpkh_address(address: &Address) -> Result<(), Error> {
    if address.address_type() == Some(AddressType::P2wpkh) {
        Ok(())
    } else {
        Err(Error::OnlySupportP2wpkhSign)
    }
}

// A mock for rubber-stamping ECDSA signatures for P2PKH
pub async fn mock_signer(
    _derivation_path: Vec<Vec<u8>>,
    _key_id: EcdsaKeyId,
    _message_hash: Vec<u8>,
) -> Vec<u8> {
    vec![255; 64]
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

/// Sign a transaction with P2WPKH address
/// NOTE: Only support P2WPKH
pub async fn sign_transaction_p2wpkh(
    public_key: &[u8],
    sender: &Address,
    mut tx_info: TransactionInfo,
    input_amounts: &[Amount],
    sighash_type: EcdsaSighashType,
    key_id: EcdsaKeyId,
    derivation_path: Vec<Vec<u8>>,
) -> Result<TransactionInfo, Error> {
    // Check if the sender is P2WPKH
    validate_p2wpkh_address(sender)?;

    let tx_clone = tx_info.tx.clone();

    let mut sighasher = sighash::SighashCache::new(&tx_clone);

    for (index, input) in tx_info.tx.input.iter_mut().enumerate() {
        let value = input_amounts.get(index).unwrap();

        let sighash = sighasher
            .p2wpkh_signature_hash(
                index,
                &sender.script_pubkey(),
                value.to_owned(),
                sighash_type,
            )
            .expect("Creating p2wpkh sighash failed");

        let signature_bytes = ecdsa::sign_with_ecdsa(
            derivation_path.clone(),
            key_id.clone(),
            sighash.to_byte_array().to_vec(),
        )
        .await?;

        // Convert the signature to DER format.
        let mut der_signature = sign_to_der(signature_bytes);
        der_signature.push(sighash_type.to_u32() as u8);

        let witness = vec![der_signature, public_key.to_vec()];

        input.witness = Witness::from_slice(&witness);
    }

    TransactionInfo::new(
        tx_clone,
        tx_info.witness_script.clone(),
        tx_info.sig_hashes.clone(),
    )
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

pub fn network_to_string(network: BitcoinNetwork) -> String {
    match network {
        BitcoinNetwork::Mainnet => "mainnet",
        BitcoinNetwork::Testnet => "testnet",
        BitcoinNetwork::Regtest => "regtest",
    }
    .to_string()
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
