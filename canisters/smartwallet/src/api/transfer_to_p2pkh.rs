use base::utils::{principal_to_derivation_path, sign_to_der};
use base::{constants::DEFAULT_FEE_MILLI_SATOSHI, utils::str_to_bitcoin_address};
use bitcoin::script::{Builder, PushBytesBuf};
use bitcoin::{
    absolute::LockTime, hashes::Hash, transaction::Version, Address, Amount, OutPoint, Script,
    Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};
use bitcoin::{consensus, sighash, AddressType, EcdsaSighashType};
use candid::Principal;
use ic_cdk::api::management_canister::{
    bitcoin::{
        bitcoin_get_current_fee_percentiles, BitcoinNetwork, GetCurrentFeePercentilesRequest,
        MillisatoshiPerByte, Satoshi, Utxo,
    },
    ecdsa::EcdsaKeyId,
};

use crate::domain::Metadata;
use crate::error::WalletError;

pub(super) async fn serve(
    owner: Principal,
    metadata: Metadata,
    recipient: String,
    amount: u64,
) -> Result<String, WalletError> {
    send_p2pkh_transaction(
        metadata.ecdsa_key_id,
        metadata.network,
        principal_to_derivation_path(owner),
        recipient,
        amount,
    )
    .await
    .map(|txid| txid.to_string())
}

/// Send a transaction to bitcoin network that transfer the given amount and recipient
/// and the sender is the canister itself
async fn send_p2pkh_transaction(
    key_id: EcdsaKeyId,
    network: BitcoinNetwork,
    derivation_path: Vec<Vec<u8>>,
    recipient: String,
    amount: Satoshi,
) -> Result<Txid, WalletError> {
    // Get fee percentiles from ic api
    let fee_percentiles =
        bitcoin_get_current_fee_percentiles(GetCurrentFeePercentilesRequest { network })
            .await
            .unwrap()
            .0;

    let fee_per_byte = if fee_percentiles.is_empty() {
        // There are no fee percentiles if network is regtest. use default fee
        DEFAULT_FEE_MILLI_SATOSHI
    } else {
        // Choose the 50th percentile if len > 50
        if fee_percentiles.len() >= 50 {
            fee_percentiles[50]
        } else {
            *fee_percentiles.last().unwrap()
        }
    };

    // Fetch public key, p2pkh address, and utxos
    // TODO: replace with stable store value
    let sender_public_key =
        base::ecdsa::public_key(derivation_path.clone(), key_id.clone(), None).await?;

    // TODO: replace with stable store value
    let sender_address = base::bitcoins::public_key_to_p2pkh_address(network, &sender_public_key);
    ic_cdk::print(format!(
        "Sender address: {sender_address:?} ---------------------- \n"
    ));

    // Fetching UTXOs
    ic_cdk::print("Fetching UTXOs... \n");

    let sender_address = str_to_bitcoin_address(&sender_address, network)?;

    // TODO: UTXOs maybe very large, need to paginate
    let utxos = base::bitcoins::get_utxos(sender_address.to_string(), network, None)
        .await?
        .utxos;

    let recipient = str_to_bitcoin_address(&recipient, network)?;

    // Build transaction
    let tx = build_transaction(
        &sender_public_key,
        &sender_address,
        &utxos,
        &recipient,
        amount,
        fee_per_byte,
    )
    .await?;

    // let tx_bytes = consensus::serialize(&tx);

    // Sign the transaction
    let signed_tx = sign_transaction_p2pkh(
        &sender_public_key,
        &sender_address,
        tx,
        key_id,
        derivation_path,
        base::ecdsa::sign_with_ecdsa_uncheck,
    )
    .await?;

    let signed_tx_bytes = consensus::serialize(&signed_tx);
    ic_cdk::print(format!("Signed tx: {:?} \n", hex::encode(&signed_tx_bytes)));

    let txid = signed_tx.compute_txid();

    ic_cdk::print(format!("Sending transaction... {txid:?}\n"));

    base::bitcoins::send_transaction(signed_tx_bytes, network).await?;

    ic_cdk::print("Transaction sent! \n");

    Ok(txid)
}

/// Build a transaction using given parameters
/// NOTE: There's a chicken-and-egg problem when compute the fee for a transaction.
/// but we need to know the proper fee in order to figure   out the inputs needed for the transaction
///
/// Solving this problem is iteratively now, we start with a fee of zero, build and sign a transactoin,
/// find what its size is, and then update the fee, rebuild the transaction, until the fee is correct.
async fn build_transaction(
    public_key: &[u8],
    sender: &Address,
    utxos: &[Utxo],
    recipient: &Address,
    amount: Satoshi,
    fee_per_byte: MillisatoshiPerByte,
) -> Result<Transaction, WalletError> {
    ic_cdk::print("Building transaction ... \n");

    let mut total_fee = 0;

    loop {
        let tx = calc_fee_and_build_transaction(sender, utxos, recipient, amount, total_fee)?;

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

/// Calculate the fee for a given arguments, and build the transaction with argument and fee if the amounts in utxos are enough
/// Returns the transaction if success, otherwise return `InsufficientFunds`
fn calc_fee_and_build_transaction(
    sender: &Address,
    utxos: &[Utxo],
    recipient: &Address,
    amount: Satoshi,
    fee: Satoshi,
) -> Result<Transaction, WalletError> {
    // Assume that any amount below this threshold is dust.
    const DUST_THRESHOLD: u64 = 1_000;

    // TODO: Optimize UTXO selection
    // Select which UTXOs to spend. We naively spend the oldest available UTXOs,
    // even if they were previously spent in a transaction. This isn't a
    // problem as long as at most one transaction is created per block and
    // we're using min_confirmations of 1.
    let mut utxos_to_spend = vec![];

    // The total spent amount is included fee
    let mut input_amount_satoshi = 0;
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
        return Err(WalletError::InsufficientFunds);
    }

    let input: Vec<TxIn> = utxos_to_spend
        .into_iter()
        .map(|u| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_raw_hash(Hash::from_slice(&u.outpoint.txid).unwrap()),
                vout: u.outpoint.vout,
            },
            sequence: Sequence::MAX,
            witness: Witness::new(),
            script_sig: Script::new().to_owned(),
        })
        .collect();

    let mut output = vec![TxOut {
        script_pubkey: recipient.script_pubkey(),
        value: Amount::from_sat(amount),
    }];

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
async fn sign_transaction_p2pkh<SignFun, Fut>(
    public_key: &[u8],
    sender: &Address,
    mut tx: Transaction,
    key_id: EcdsaKeyId,
    derivation_path: Vec<Vec<u8>>,
    signer: SignFun,
) -> Result<Transaction, WalletError>
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

fn validate_p2pkh_address(address: &Address) -> Result<(), WalletError> {
    if address.address_type() == Some(AddressType::P2pkh) {
        Ok(())
    } else {
        Err(WalletError::OnlySupportP2pkhSign)
    }
}

// A mock for rubber-stamping ECDSA signatures for P2PKH
async fn mock_signer(
    _derivation_path: Vec<Vec<u8>>,
    _key_id: EcdsaKeyId,
    _message_hash: Vec<u8>,
) -> Vec<u8> {
    vec![255; 64]
}
