use base::tx::RawTransactionInfo;
use candid::Principal;

use crate::domain::SelfCustodyKey;
use crate::{domain::request::TransferRequest, error::WalletError};

use super::{get_metadata, get_raw_wallet};

pub(super) async fn serve(
    caller: Principal,
    req: TransferRequest,
) -> Result<RawTransactionInfo, WalletError> {
    let metadata = get_metadata();

    // Check the caller is the owner
    if caller != metadata.owner {
        return Err(WalletError::UnAuthorized(caller.to_string()));
    }

    let network = metadata.network;
    let key_id = metadata.ecdsa_key_id;
    let steward_canister = metadata.steward_canister;
    let wallet_key = SelfCustodyKey {
        network,
        owner: caller,
        steward_canister,
        wallet_type: base::domain::WalletType::Single,
        address_type: base::domain::AddressType::P2wsh,
    };

    let wallet_opt = get_raw_wallet(&wallet_key);

    let wallet = if let Some(wallet) = wallet_opt {
        wallet
    } else {
        return Err(WalletError::WalletNotFound(format!("{:?}", wallet_key)));
    };

    let receiver_addresses = req
        .addresses
        .as_slice()
        .iter()
        .map(|a| a.as_str())
        .collect::<Vec<&str>>();

    let mut tx_info = base::utils::build_unsigned_transaction_auto(
        wallet.into(),
        &req.amounts,
        &receiver_addresses,
        network,
    )
    .await;

    tx_info = base::utils::sign_transaction(
        tx_info?,
        &[caller.as_slice().to_vec()],
        key_id,
        base::domain::MultiSigIndex::First,
    )
    .await;

    // TODO: Log transactions
    // STATE.with(|s| {
    //     let mut state = s.borrow_mut();
    //     state.transactions.push(tx_info.clone());
    // });
    tx_info.map(RawTransactionInfo::from).map_err(|e| e.into())
}
