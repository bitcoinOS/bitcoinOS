use base::utils::principal_to_derivation_path;
use candid::Principal;

use crate::domain::{Metadata, RawWallet, SelfCustodyKey};
use crate::{domain::request::TransferRequest, error::WalletError};

use super::{build_and_append_transaction_log, get_raw_wallet};

pub(super) async fn serve(
    owner: Principal,
    metadata: Metadata,
    req: TransferRequest,
) -> Result<String, WalletError> {
    let wallet = get_raw_wallet_opt(&metadata, owner)?;
    let network = metadata.network;

    let tx_req = req.validate_address(network)?;

    // Log transfer info
    build_and_append_transaction_log(req.txs)?;

    // build transaction

    let key_id = metadata.ecdsa_key_id;

    let tx_info =
        base::utils::build_unsigned_transaction_auto(wallet.into(), tx_req, network).await?;

    let tx_info =
        base::utils::sign_transaction_single(tx_info, &principal_to_derivation_path(owner), key_id)
            .await?;

    let txid = base::utils::send_transaction(&tx_info, network).await?;

    Ok(txid.to_string())
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
