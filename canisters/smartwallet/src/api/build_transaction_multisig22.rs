use base::tx::RawTransactionInfo;
use candid::Principal;

use crate::domain::{Metadata, RawWallet, SelfCustodyKey};
use crate::{domain::request::TransferRequest, error::WalletError};

use super::{build_and_append_transaction_log, get_raw_wallet};

pub(super) async fn serve(
    owner: Principal,
    metadata: Metadata,
    tx_req: TransferRequest,
) -> Result<RawTransactionInfo, WalletError> {
    let wallet = get_raw_wallet_opt(&metadata, owner)?;

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
        &[owner.as_slice().to_vec()],
        key_id,
        base::domain::MultiSigIndex::First,
    )
    .await;

    // Log transfer info
    build_and_append_transaction_log(tx_req.txs)?;

    tx_info.map(RawTransactionInfo::from).map_err(|e| e.into())
}

fn get_raw_wallet_opt(metadata: &Metadata, caller: Principal) -> Result<RawWallet, WalletError> {
    let network = metadata.network;
    let steward_canister = metadata.steward_canister;
    let wallet_key = SelfCustodyKey {
        network,
        owner: caller,
        steward_canister,
        wallet_type: base::domain::WalletType::MultiSig22,
        address_type: base::domain::AddressType::P2wsh,
    };

    get_raw_wallet(&wallet_key).ok_or(WalletError::WalletNotFound(format!("{:?}", wallet_key)))
}
