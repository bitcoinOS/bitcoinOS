use bitcoin::{consensus, hashes::Hash, Address, Amount, ScriptBuf, SegwitV0Sighash, Transaction};
use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{domain::request::TransferInfo, error::Error, utils::check_tx_hashes_len};

#[derive(Debug, Clone)]
pub struct RecipientAmount {
    // A bitcoin address
    pub recipient: Address,
    pub amount: Amount,
}

impl From<&RecipientAmount> for TransferInfo {
    fn from(value: &RecipientAmount) -> Self {
        Self {
            recipient: value.recipient.to_string(),
            amount: value.amount.to_sat(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecipientAmountVec {
    pub txs: Vec<RecipientAmount>,
}

impl RecipientAmountVec {
    pub fn iter(&self) -> impl Iterator<Item = &RecipientAmount> {
        self.txs.iter()
    }
}

impl IntoIterator for RecipientAmountVec {
    type Item = RecipientAmount;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.txs.into_iter()
    }
}

pub struct TransactionInfo {
    pub tx: Transaction,
    pub witness_script: ScriptBuf,
    pub sig_hashes: Vec<SegwitV0Sighash>,
}

impl TransactionInfo {
    pub fn new(
        tx: Transaction,
        witness_script: ScriptBuf,
        sig_hashes: Vec<SegwitV0Sighash>,
    ) -> Result<Self, Error> {
        check_tx_hashes_len(&tx, &sig_hashes)?;

        Ok(TransactionInfo {
            tx,
            witness_script,
            sig_hashes,
        })
    }

    // Get the transaction
    pub fn tx(&self) -> &Transaction {
        &self.tx
    }

    // Get the witness script
    pub fn witness_script(&self) -> &ScriptBuf {
        &self.witness_script
    }

    // Get the sighashes
    pub fn sig_hashes(&self) -> &Vec<SegwitV0Sighash> {
        &self.sig_hashes
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct RawTransactionInfo {
    pub tx: Vec<u8>,
    pub witness_script: Vec<u8>,
    pub sig_hashes: Vec<Vec<u8>>,
}

impl TryFrom<RawTransactionInfo> for TransactionInfo {
    type Error = Error;

    fn try_from(tx_info: RawTransactionInfo) -> Result<Self, Self::Error> {
        let tx = consensus::deserialize(&tx_info.tx).map_err(|_| Error::DeserializeError)?;
        let witness_script = ScriptBuf::from(tx_info.witness_script);
        let sig_hashes: Vec<SegwitV0Sighash> = tx_info
            .sig_hashes
            .into_iter()
            .map(|s| SegwitV0Sighash::from_byte_array(s.try_into().unwrap()))
            .collect();

        Ok(Self {
            tx,
            witness_script,
            sig_hashes,
        })
    }
}

impl From<TransactionInfo> for RawTransactionInfo {
    fn from(tx_info: TransactionInfo) -> Self {
        RawTransactionInfo {
            tx: consensus::serialize(&tx_info.tx),
            witness_script: tx_info.witness_script.into_bytes(),
            sig_hashes: tx_info
                .sig_hashes
                .iter()
                .map(|s| s.as_byte_array().to_vec())
                .collect(),
        }
    }
}
