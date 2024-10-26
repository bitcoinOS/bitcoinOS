use std::str::FromStr;

use bitcoin::{
    absolute::LockTime, consensus, hashes::Hash, sighash, Address, AddressType, Amount,
    CompressedPublicKey, EcdsaSighashType, OutPoint, PublicKey, ScriptBuf, Sequence, Transaction,
    TxIn, TxOut, Txid, Witness,
};
use candid::Principal;
use ic_cdk::api::management_canister::{
    bitcoin::{BitcoinNetwork, MillisatoshiPerByte},
    ecdsa::EcdsaKeyId,
};

use crate::{
    constants::DEFAULT_FEE_MILLI_SATOSHI,
    domain::{response::Utxo, EcdsaKeyIdAndDerivationPath, Wallet, WalletType},
    ecdsa,
    error::Error,
    tx::RecipientAmount,
    utils::{mock_ecdsa_signer, principal_to_derivation_path, sign_to_der, to_bitcoin_network},
};

/// Create a new P2WPKH wallet with given arguments
pub async fn create_p2wpkh_wallet(
    owner: Principal,
    key_id: EcdsaKeyId,
    network: BitcoinNetwork,
) -> Result<Wallet, Error> {
    let derivation_path = principal_to_derivation_path(owner);
    let public_key = ecdsa::public_key(derivation_path.clone(), key_id, None).await?;

    create_p2wpkh_wallet_with_pk(derivation_path, &public_key, network).await
}

/// Returns the P2WPKH address of this canister at the given derivation path.
pub async fn create_p2wpkh_wallet_with_pk(
    derivation_path: Vec<Vec<u8>>,
    public_key: &[u8],
    network: BitcoinNetwork,
) -> Result<Wallet, Error> {
    let public_key =
        PublicKey::from_slice(public_key).map_err(|e| Error::Secp256k1Error(e.to_string()))?;

    let compressed_pk = CompressedPublicKey(public_key.inner);

    let address = bitcoin::Address::p2wpkh(&compressed_pk, to_bitcoin_network(network));

    let witness_script = ScriptBuf::p2wpkh_script_code(compressed_pk.wpubkey_hash());

    Ok(Wallet {
        witness_script,
        address,
        derivation_path,
        wallet_type: WalletType::Single,
    })
}

/// Build a transaction using given parameters
pub async fn build_unsigned_p2wpkh_transaction(
    network: BitcoinNetwork,
    address: &Address,
    pk_bytes: &[u8],
    derivation_path: Vec<Vec<u8>>,
    txs: &[RecipientAmount],
    sighash_type: EcdsaSighashType,
) -> Result<(Transaction, Vec<Amount>), Error> {
    let fee_per_byte = super::get_fee_per_byte(network, DEFAULT_FEE_MILLI_SATOSHI).await?;

    // Fetch UTXOs
    ic_cdk::print("Fetching Utxos for p2wpkh ... \n");

    // FIXME: UTXOs maybe very large, need to paginate
    let utxos = super::get_utxos(address.to_string(), network, None)
        .await?
        .utxos;

    build_p2wpkh_transaction_info(
        address,
        pk_bytes,
        derivation_path,
        &utxos,
        txs,
        fee_per_byte,
        sighash_type,
    )
    .await
}

pub async fn build_p2wpkh_transaction_info(
    address: &Address,
    pk_bytes: &[u8],
    derivation_path: Vec<Vec<u8>>,
    utxos: &[Utxo],
    txs: &[RecipientAmount],
    fee_per_byte: MillisatoshiPerByte,
    sighash_type: EcdsaSighashType,
) -> Result<(Transaction, Vec<Amount>), Error> {
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
        let tx_and_input_amounts =
            build_p2wpkh_transaction_with_fee(address, utxos, txs, total_fee)?;

        // Sign the transaction. In this case, we only care about the size
        // of the signed transaction, so we use a mock signer here for efficiency.
        // let signed_transaction = fake_p2wpkh_signatures(&tx_and_input_amounts.0, sighash_type);
        let key_id_derivation_path = EcdsaKeyIdAndDerivationPath {
            derivation_path: derivation_path.clone(),
            key_id: EcdsaKeyId::default(),
        };

        let signed_transaction = sign_transaction_p2wpkh(
            pk_bytes,
            address,
            tx_and_input_amounts.0.clone(),
            &tx_and_input_amounts.1,
            sighash_type,
            key_id_derivation_path,
            mock_ecdsa_signer,
        )
        .await?;

        let signed_tx_bytes_len = consensus::serialize(&signed_transaction).len() as u64;

        if (signed_tx_bytes_len * fee_per_byte) / 1000 == total_fee {
            ic_cdk::print(format!("Transaction built with fee {}.\n", total_fee));
            return Ok(tx_and_input_amounts);
        } else {
            total_fee = (signed_tx_bytes_len * fee_per_byte) / 1000;
        }
    }
}

/// Build a segwit transaction and calculate the fee
pub fn build_p2wpkh_transaction_with_fee(
    address: &Address,
    utxos: &[Utxo],
    txs: &[RecipientAmount],
    fee: MillisatoshiPerByte,
) -> Result<(Transaction, Vec<Amount>), Error> {
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
            script_pubkey: address.script_pubkey(),
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
    // let sig_hashes = build_p2wpkh_transaction_sighashes(
    //     &transaction,
    //     my_wallet,
    //     &input_amounts,
    // );
    // let sig_hashes = vec![];

    // Return all the data required to sign the transaction.
    // let tx_info = TransactionInfo::new(transaction, my_wallet.witness_script.clone(), sig_hashes)?;
    Ok((transaction, input_amounts))
}

/// Computes the sighashes for each input of the given transaction.
// /// The sighash is computed using the given witness script and input amounts.
// pub fn build_p2wpkh_transaction_sighashes(
//     transaction: &Transaction,
//     wallet: &Wallet,
//     input_amounts: &[Amount],
// ) -> Vec<SegwitV0Sighash> {
//     if transaction.input.len() != input_amounts.len() {
//         panic!("Transaction inputs and amounts must have the same length.");
//     }

//     let mut sig_hashes = vec![];

//     let txclone = transaction.clone();
//     ic_cdk::print(format!("tx in sighahses building is {:#?} ------------------\n", transaction));

//     let mut cache = sighash::SighashCache::new(&txclone);

//     for (input_index, _input) in transaction.input.iter().enumerate() {
//         let value = input_amounts.get(input_index).unwrap();

//         // Compute the sighash for this input using the witness script from the user wallet.
//         let sighash = cache
//             .p2wpkh_signature_hash(
//                 input_index,
//                 &wallet.address.script_pubkey(),
//                 value.to_owned(),
//                 EcdsaSighashType::All,
//             )
//             .expect("failed to compute sighash");

//         sig_hashes.push(sighash);
//     }

//     sig_hashes
// }

// Fake the signatures of the smart wallet and the steward canister.
// pub fn fake_p2wpkh_signatures(tx: &Transaction, signhash_type: EcdsaSighashType) -> Transaction {
//     let mut transaction = tx.clone();

//     for input in transaction.input.iter_mut() {
//         // Clear any previous witness
//         input.witness.clear();

//         // Fake signature using an arbitrary array of bytes.
//         let sec1_signature = vec![255; 64];

//         // Convert the signature to DER format.
//         let mut der_signature = sign_to_der(sec1_signature);
//         der_signature.push(signhash_type.to_u32() as u8);

//         let witness = vec![der_signature, vec![]];

//         input.witness = Witness::from_slice(&witness);
//     }

//     // TransactionInfo::new(
//     //     transaction,
//     //     transaction_info.witness_script.clone(),
//     //     transaction_info.sig_hashes.clone(),
//     // )
//     transaction
// }

/// Sign a transaction with P2WPKH address
/// NOTE: Only support P2WPKH
// pub async fn sign_transaction_p2wpkh(
//     public_key: &[u8],
//     sender: &Address,
//     mut tx: Transaction,
//     input_amounts: &[Amount],
//     sighash_type: EcdsaSighashType,
//     key_id: EcdsaKeyId,
//     derivation_path: Vec<Vec<u8>>,
// ) -> Result<Transaction, Error> {
//     // Check if the sender is P2WPKH
//     validate_p2wpkh_address(sender)?;

//     let tx_clone = tx.clone();

//     let mut sighasher = sighash::SighashCache::new(&tx_clone);

//     for (index, input) in tx.input.iter_mut().enumerate() {
//         let value = input_amounts.get(index).unwrap();

//         let sighash = sighasher
//             .p2wpkh_signature_hash(
//                 index,
//                 &sender.script_pubkey(),
//                 value.to_owned(),
//                 sighash_type,
//             )
//             .expect("Creating p2wpkh sighash failed");

//         let signature_bytes = ecdsa::sign_with_ecdsa(
//             derivation_path.clone(),
//             key_id.clone(),
//             sighash.to_byte_array().to_vec(),
//         )
//         .await?;

//         // Convert the signature to DER format.
//         let mut der_signature = sign_to_der(signature_bytes);
//         der_signature.push(sighash_type.to_u32() as u8);

//         let witness = vec![der_signature, public_key.to_vec()];

//         input.witness = Witness::from_slice(&witness);
//     }

//     Ok(tx)

// }

pub async fn sign_transaction_p2wpkh<SignFun, Fut>(
    public_key: &[u8],
    sender: &Address,
    mut tx: Transaction,
    input_amounts: &[Amount],
    sighash_type: EcdsaSighashType,
    key_id_derivation_path: EcdsaKeyIdAndDerivationPath,
    signer: SignFun,
) -> Result<Transaction, Error>
where
    SignFun: Fn(Vec<Vec<u8>>, EcdsaKeyId, Vec<u8>) -> Fut,
    Fut: std::future::Future<Output = Vec<u8>>,
{
    // Check if the sender is P2WPKH
    validate_p2wpkh_address(sender)?;

    let tx_clone = tx.clone();

    let mut sighasher = sighash::SighashCache::new(&tx_clone);

    for (index, input) in tx.input.iter_mut().enumerate() {
        let value = input_amounts.get(index).unwrap();

        let sighash = sighasher
            .p2wpkh_signature_hash(
                index,
                &sender.script_pubkey(),
                value.to_owned(),
                sighash_type,
            )
            .expect("Creating p2wpkh sighash failed");

        let signature_bytes = signer(
            key_id_derivation_path.derivation_path.clone(),
            key_id_derivation_path.key_id.clone(),
            sighash.to_byte_array().to_vec(),
        )
        .await;

        // Convert the signature to DER format.
        let mut der_signature = sign_to_der(signature_bytes);
        der_signature.push(sighash_type.to_u32() as u8);

        let witness = vec![der_signature, public_key.to_vec()];

        input.witness = Witness::from_slice(&witness);
    }

    Ok(tx)

    // TransactionInfo::new(
    //     tx_info.tx,
    //     tx_info.witness_script.clone(),
    //     tx_info.sig_hashes.clone(),
    // )
}

pub fn validate_p2wpkh_address(address: &Address) -> Result<(), Error> {
    if address.address_type() == Some(AddressType::P2wpkh) {
        Ok(())
    } else {
        Err(Error::OnlySupportP2wpkhSign)
    }
}
