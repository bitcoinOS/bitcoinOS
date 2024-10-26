use std::str::FromStr;

use bitcoin::{
    absolute::LockTime, consensus, hashes::Hash, Amount, EcdsaSighashType, OutPoint, ScriptBuf,
    SegwitV0Sighash, Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};
use ic_cdk::api::management_canister::{
    bitcoin::{BitcoinNetwork, MillisatoshiPerByte},
    ecdsa::EcdsaKeyId,
};

use crate::{
    constants::{DEFAULT_FEE_MILLI_SATOSHI, DUST_THRESHOLD},
    domain::{response::Utxo, MultiSigIndex, Wallet},
    ecdsa,
    error::Error,
    tx::{RecipientAmount, TransactionInfo},
    utils::sign_to_der,
};

/// Build a p2wsh multisig 2-2 transction
pub async fn build_unsigned_transaction_p2wsh_multisig22(
    my_wallet: &Wallet,
    network: BitcoinNetwork,
    txs: &[RecipientAmount],
    sighash_type: EcdsaSighashType,
) -> Result<TransactionInfo, Error> {
    let fee_per_bytes = super::get_fee_per_byte(network, DEFAULT_FEE_MILLI_SATOSHI).await?;

    ic_cdk::print(format!(
        "Got fee per bytes is: {fee_per_bytes:?} ------------ \n"
    ));

    // Fetch UTXOs
    ic_cdk::print("Fetching Utxos ------------- \n");

    // FIXME: UTXOs maybe very large, need to paginate
    let utxos = super::get_utxos(my_wallet.address.to_string(), network, None)
        .await?
        .utxos;

    build_p2wsh_multisig22_transaction_info(my_wallet, &utxos, txs, fee_per_bytes, sighash_type)
        .await
}

pub async fn build_p2wsh_multisig22_transaction_info(
    my_wallet: &Wallet,
    utxos: &[Utxo],
    txs: &[RecipientAmount],
    fee_per_byte: MillisatoshiPerByte,
    sighash_type: EcdsaSighashType,
) -> Result<TransactionInfo, Error> {
    ic_cdk::print("Building transaction ----------- \n");

    let mut total_fee = 0;

    loop {
        let tx_info =
            build_transaction_with_fee_p2wsh_multisig_22(my_wallet, utxos, txs, total_fee)?;

        // Sign the transaction.
        // We only care about the size of the signed transaction, so we use a mock signer here for efficiency.
        let signed_tx = fake_signatures_p2wsh_multisig22(&tx_info, sighash_type)?.tx;

        let signed_tx_bytes_len = consensus::serialize(&signed_tx).len() as u64;

        if (signed_tx_bytes_len * fee_per_byte) / 1000 == total_fee {
            ic_cdk::print(format!("Transaction built with fee {}.", total_fee));

            return Ok(tx_info);
        } else {
            total_fee = (signed_tx_bytes_len * fee_per_byte) / 1000;
        }
    }
}

// Build a transaction to send the given amount of satoshis to the
// destination address, with the given fee.
fn build_transaction_with_fee_p2wsh_multisig_22(
    my_wallet: &Wallet,
    utxos: &[Utxo],
    txs: &[RecipientAmount],
    fee: u64,
) -> Result<TransactionInfo, Error> {
    let mut utxos_to_spend = vec![];

    // Segwit signature need the input amount for eacth input
    let mut input_amounts = vec![];

    // Total amount of output amounts
    let total_outputs: u64 = txs.iter().map(|r| r.amount.to_sat()).sum();

    let mut total_spent = 0;

    // Auto select the output from the oldest
    for utxo in utxos.iter().rev() {
        total_spent += utxo.value;

        utxos_to_spend.push(utxo);
        input_amounts.push(Amount::from_sat(utxo.value));

        // We have enough inputs to cover the amount and fee.
        if total_spent >= total_outputs + fee {
            break;
        }
    }

    let outputs_and_fee = total_outputs + fee;

    ic_cdk::print(format!(
        "The total spent is: {total_spent:?} -----------\n "
    ));
    ic_cdk::print(format!(
        "The outputs and fee total is {outputs_and_fee:?} -------------\n "
    ));

    // Check that we have enough balance to cover the amount we want to spend.
    if total_spent < outputs_and_fee {
        return Err(Error::InsufficientFunds);
    }

    // Build the transaction's inputs from previous utxos.
    let inputs: Vec<TxIn> = utxos_to_spend
        .into_iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_str(&utxo.outpoint.txid).unwrap(),
                vout: utxo.outpoint.vout,
            },
            sequence: Sequence::MAX, // 0xffffffff,
            witness: Witness::new(),
            script_sig: ScriptBuf::new(),
        })
        .collect();

    // Build the transaction's output from recipients and amounts
    let mut outputs: Vec<TxOut> = txs
        .iter()
        .map(|r| TxOut {
            script_pubkey: r.recipient.script_pubkey(),
            value: r.amount,
        })
        .collect();

    let remaining_amount = total_spent - total_outputs - fee;

    if remaining_amount >= DUST_THRESHOLD {
        outputs.push(TxOut {
            script_pubkey: my_wallet.address.script_pubkey(),
            value: Amount::from_sat(remaining_amount),
        });
    }

    let tx = Transaction {
        input: inputs,
        output: outputs,
        lock_time: LockTime::ZERO,
        version: bitcoin::blockdata::transaction::Version::ONE,
    };

    // Compute the sighashes for each input.
    let sig_hashes = build_transaction_sighashes_p2wsh_multisig22(
        &tx,
        &my_wallet.witness_script,
        input_amounts.clone(),
    )?;

    // Return all the data required to sign the transaction.
    TransactionInfo::new(tx, my_wallet.witness_script.clone(), sig_hashes)
}

// Computes the sighashes for each input of the given transaction.
// The sighash is computed using the given witness script and input amounts.
fn build_transaction_sighashes_p2wsh_multisig22(
    tx: &Transaction,
    witness_script: &ScriptBuf,
    input_amounts: Vec<Amount>,
) -> Result<Vec<SegwitV0Sighash>, Error> {
    if tx.input.len() != input_amounts.len() {
        // panic!("Transaction inputs and amounts must have the same length.");
        return Err(Error::AmountsAndAddressesMismatch);
    }

    let mut sig_hashes = vec![];

    let txclone = tx.clone();
    let mut cache = bitcoin::sighash::SighashCache::new(&txclone);

    for (input_index, _input) in tx.input.iter().enumerate() {
        let value = input_amounts.get(input_index).unwrap();

        // Compute the sighash for this input using the witness script from the user wallet.
        let sighash = cache
            .p2wsh_signature_hash(
                input_index,
                witness_script,
                value.to_owned(),
                EcdsaSighashType::All,
            )
            .map_err(|e| Error::P2wshSigHashError(format!("{e:?}")))?;

        sig_hashes.push(sighash);
    }

    Ok(sig_hashes)
}
// Fake the signatures of the smartwallet and the steward canister.
fn fake_signatures_p2wsh_multisig22(
    transaction_info: &TransactionInfo,
    sighash_type: EcdsaSighashType,
) -> Result<TransactionInfo, Error> {
    let mut transaction = transaction_info.tx.clone();

    for input in transaction.input.iter_mut() {
        // Clear any previous witness
        input.witness.clear();

        // Fake signature using an arbitrary array of bytes.
        let sec1_signature = vec![255; 64];

        // Convert the signature to DER format.
        let mut der_signature = sign_to_der(sec1_signature);
        der_signature.push(sighash_type.to_u32() as u8);

        // Add the signatures to the witness.
        input.witness.push(vec![]); // Placeholder for scriptSig
        input.witness.push(der_signature.clone());
        input.witness.push(der_signature);
        input
            .witness
            .push(transaction_info.witness_script.clone().into_bytes());
    }

    TransactionInfo::new(
        transaction,
        transaction_info.witness_script.clone(),
        transaction_info.sig_hashes.clone(),
    )
}

// Add a signature to the given transaction.
// The signature is computed using the given key and derivation path.
// The signature index indicates whether it is the first or last signature.
// Warning: this function assumes that the sender of the transaction is the P2WSH
// address that corresponds to the witness script of the user wallet. Do not use
// this function to sign transactions that are not sent from this address.
pub async fn sign_transaction_p2wsh_multisig22(
    transaction_info: &TransactionInfo,
    key_id: EcdsaKeyId,
    derivation_path: &Vec<Vec<u8>>,
    signature_index: MultiSigIndex,
    sighash_type: EcdsaSighashType,
) -> Result<TransactionInfo, Error> {
    let mut tx = transaction_info.tx.clone();

    // Sign each input of the transaction.
    for (index, input) in tx.input.iter_mut().enumerate() {
        // If it is the first signature, clear any previous witness script
        // and add a placeholder for the scriptSig.
        if signature_index == MultiSigIndex::First {
            input.witness.clear();
            input.witness.push(vec![]);
        }

        // Get the sighash for this input.
        let sighash = transaction_info.sig_hashes.get(index).unwrap();

        // Sign the sighash with the given key and derivation path.
        let sec1_signature = ecdsa::sign_with_ecdsa(
            derivation_path.to_owned(),
            key_id.clone(),
            sighash.to_byte_array().to_vec(),
        )
        .await?;

        // Convert the signature to DER format.
        let mut der_signature = sign_to_der(sec1_signature);
        der_signature.push(sighash_type as u8);

        // Add the signature to the witness.
        input.witness.push(der_signature);

        // If it is the last signature, add the witness script.
        if signature_index == MultiSigIndex::Second {
            input
                .witness
                .push(transaction_info.witness_script.clone().into_bytes());
        }
    }

    // Return the transaction info with the updated transaction.
    TransactionInfo::new(
        tx,
        transaction_info.witness_script.clone(),
        transaction_info.sig_hashes.clone(),
    )
}
