use base::tx::RawTransactionInfo;
use candid::Principal;

use crate::context::STATE;
use crate::domain::SelfCustodyKey;
use crate::{domain::request::TransferRequest, error::WalletError};

pub(super) async fn serve(
    caller: Principal,
    req: TransferRequest,
) -> Result<RawTransactionInfo, WalletError> {
    let metadata = STATE.with(|s| s.borrow().metadata.get().clone());

    let network = metadata.network;
    let key_name = metadata.key_name;
    let steward_canister = metadata.steward_canister;
    let wallet_key = SelfCustodyKey {
        network,
        owner: caller,
        steward_canister,
    };

    let wallet_opt = STATE.with(|s| {
        let state = s.borrow();
        state.raw_wallet.get(&wallet_key)
    });

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
        &key_name,
        &[caller.as_slice().to_vec()],
        base::domain::MultiSigIndex::First,
    )
    .await;

    /// TODO: Log transactions
    // STATE.with(|s| {
    //     let mut state = s.borrow_mut();
    //     state.transactions.push(tx_info.clone());
    // });
    tx_info.map(RawTransactionInfo::from).map_err(|e| e.into())
}
