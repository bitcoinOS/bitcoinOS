use base::tx::RawTransactionInfo;
use base::utils::{ic_caller, ic_time};
use candid::Principal;

use crate::domain::{Metadata, RawWallet, SelfCustodyKey, TransactionLog};
use crate::{domain::request::TransferRequest, error::WalletError};

use super::{append_transaction_log, get_raw_wallet, validate_owner};

pub(super) async fn serve(
    caller: Principal,
    tx_req: TransferRequest,
) -> Result<RawTransactionInfo, WalletError> {
    let metadata = validate_owner(caller)?;

    let wallet = get_raw_wallet_opt(&metadata, caller)?;

    // build transaction
    let network = metadata.network;
    let key_id = metadata.ecdsa_key_id;

    let mut tx_info = base::utils::build_unsigned_transaction_auto(
        wallet.into(),
        tx_req.validate_address(network)?,
        network,
    )
    .await;

    // Signature transaction
    tx_info = base::utils::sign_transaction_multisig22(
        tx_info?,
        &[caller.as_slice().to_vec()],
        key_id,
        base::domain::MultiSigIndex::First,
    )
    .await;

    // Log transfer info
    let sender = ic_caller();
    let send_time = ic_time();
    let log = TransactionLog {
        txs: tx_req.txs,
        sender,
        send_time,
    };

    append_transaction_log(log)?;

    tx_info.map(RawTransactionInfo::from).map_err(|e| e.into())
}

fn get_raw_wallet_opt(metadata: &Metadata, caller: Principal) -> Result<RawWallet, WalletError> {
    let network = metadata.network;
    let steward_canister = metadata.steward_canister;
    let wallet_key = SelfCustodyKey {
        network,
        owner: caller,
        steward_canister,
        wallet_type: base::domain::WalletType::Single,
        address_type: base::domain::AddressType::P2wsh,
    };

    get_raw_wallet(&wallet_key).ok_or(WalletError::WalletNotFound(format!("{:?}", wallet_key)))
}
