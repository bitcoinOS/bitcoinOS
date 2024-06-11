use std::str::FromStr;

use bitcoin::{
    absolute::LockTime, consensus, sighash, Amount, EcdsaSighashType, OutPoint, Script, ScriptBuf,
    SegwitV0Sighash, Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};
use ic_cdk::api::management_canister::bitcoin::MillisatoshiPerByte;

use crate::{
    domain::{response::Utxo, Wallet},
    error::Error,
    tx::{RecipientAmount, TransactionInfo},
    utils::sign_to_der,
};

pub async fn build_p2wpkh_transaction_info(
    my_wallet: &Wallet,
    utxos: &[Utxo],
    txs: &[RecipientAmount],
    fee_per_byte: MillisatoshiPerByte,
    sighash_type: EcdsaSighashType,
) -> Result<(TransactionInfo, Vec<Amount>), Error> {
    // We have a chicken-and-egg problem where we need to know the length
    // of the transaction in order to compute its proper fee, but we need
    // to know the proper fee in order to figure out the inputs needed for
    // the transaction.
    //
    // We solve this problem iteratively. We start with a fee of zero, build
    // and sign a transaction, see what its size is, and then update the fee,
    // rebuild the transaction, until the fee is set to the correct amount.
    ic_cdk::print("Building transaction... \n");

    let mut total_fee = 0;

    loop {
        let transaction_info = build_p2wpkh_transaction_with_fee(my_wallet, utxos, txs, total_fee)?;

        // Sign the transaction. In this case, we only care about the size
        // of the signed transaction, so we use a mock signer here for efficiency.
        let signed_transaction = fake_p2wpkh_signatures(&transaction_info.0, sighash_type)?.tx;

        let signed_tx_bytes_len = consensus::serialize(&signed_transaction).len() as u64;

        if (signed_tx_bytes_len * fee_per_byte) / 1000 == total_fee {
            ic_cdk::print(format!("Transaction built with fee {}.\n", total_fee));
            return Ok(transaction_info);
        } else {
            total_fee = (signed_tx_bytes_len * fee_per_byte) / 1000;
        }
    }
}

/// Build a segwit transaction and calculate the fee
pub fn build_p2wpkh_transaction_with_fee(
    my_wallet: &Wallet,
    utxos: &[Utxo],
    txs: &[RecipientAmount],
    fee: MillisatoshiPerByte,
) -> Result<(TransactionInfo, Vec<Amount>), Error> {
    // Assume that any amount below this threshold is dust.
    const DUST_THRESHOLD: u64 = 1_000;

    // Select which UTXOs to spend. We naively spend the oldest available UTXOs,
    // even if they were previously spent in a transaction. This isn't a
    // problem as long as at most one transaction is created per block and
    // we're using min_confirmations of 1.
    let mut utxos_to_spend = vec![];

    // segwit signature need the input amount for each input
    let mut input_amounts = vec![];

    let amount: u64 = txs.iter().map(|r| r.amount.to_sat()).sum();

    let mut total_spent = 0;

    for utxo in utxos.iter().rev() {
        total_spent += utxo.value;
        utxos_to_spend.push(utxo);
        input_amounts.push(Amount::from_sat(utxo.value));

        if total_spent >= amount + fee {
            // We have enough inputs to cover the amount we want to spend.
            break;
        }
    }

    // Check that we have enough balance to cover the amount we want to spend.
    if total_spent < amount + fee {
        return Err(Error::InsufficientFunds);
    }

    // Build the transaction's inputs from the Utxos.
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

    // Build the transaction's output from the destination address (and the change address if applicable)
    let mut outputs: Vec<TxOut> = txs
        .iter()
        .map(|r| TxOut {
            script_pubkey: r.recipient.script_pubkey(),
            value: r.amount,
        })
        .collect();

    let remaining_amount = total_spent - amount - fee;

    if remaining_amount >= DUST_THRESHOLD {
        outputs.push(TxOut {
            script_pubkey: my_wallet.address.script_pubkey(),
            value: Amount::from_sat(remaining_amount),
        });
    }

    let transaction = Transaction {
        input: inputs,
        output: outputs,
        lock_time: LockTime::ZERO,
        version: bitcoin::blockdata::transaction::Version::ONE,
    };

    // Compute the sighashes for each input.
    let sig_hashes = build_p2wpkh_transaction_sighashes(
        &transaction,
        &my_wallet.address.script_pubkey(),
        &input_amounts,
    );

    // Return all the data required to sign the transaction.
    let tx_info = TransactionInfo::new(transaction, my_wallet.witness_script.clone(), sig_hashes)?;
    Ok((tx_info, input_amounts))
}

/// Computes the sighashes for each input of the given transaction.
/// The sighash is computed using the given witness script and input amounts.
pub fn build_p2wpkh_transaction_sighashes(
    transaction: &Transaction,
    script_pubkey: &Script,
    input_amounts: &[Amount],
) -> Vec<SegwitV0Sighash> {
    if transaction.input.len() != input_amounts.len() {
        panic!("Transaction inputs and amounts must have the same length.");
    }

    let mut sig_hashes = vec![];

    let txclone = transaction.clone();
    let mut cache = sighash::SighashCache::new(&txclone);

    for (input_index, _input) in transaction.input.iter().enumerate() {
        let value = input_amounts.get(input_index).unwrap();

        // Compute the sighash for this input using the witness script from the user wallet.
        let sighash = cache
            .p2wpkh_signature_hash(
                input_index,
                script_pubkey,
                value.to_owned(),
                EcdsaSighashType::All,
            )
            .expect("failed to compute sighash");

        sig_hashes.push(sighash);
    }

    sig_hashes
}

// Fake the signatures of the custody wallet and the fiduciary canister.
pub fn fake_p2wpkh_signatures(
    transaction_info: &TransactionInfo,
    signhash_type: EcdsaSighashType,
) -> Result<TransactionInfo, Error> {
    let mut transaction = transaction_info.tx.clone();

    for input in transaction.input.iter_mut() {
        // Clear any previous witness
        input.witness.clear();

        // Fake signature using an arbitrary array of bytes.
        let sec1_signature = vec![255; 64];

        // Convert the signature to DER format.
        let mut der_signature = sign_to_der(sec1_signature);
        der_signature.push(signhash_type.to_u32() as u8);

        let witness = vec![der_signature, vec![]];

        input.witness = Witness::from_slice(&witness);
    }

    TransactionInfo::new(
        transaction,
        transaction_info.witness_script.clone(),
        transaction_info.sig_hashes.clone(),
    )
}
