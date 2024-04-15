use candid::Principal;
use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, ecdsa::EcdsaKeyId};

use crate::{
    context::STATE,
    domain::{RawWallet, SelfCustodyKey, Wallet},
    error::WalletError,
};

use super::{get_metadata, get_raw_wallet};

/// Get an exist address, or generate a new address by caller
/// Returns a address if success, or returns `CreateWalletFailed`
/// TODO: support multiple addresses for different diravation path
pub async fn serve(caller: Principal) -> Result<String, WalletError> {
    let metadata = get_metadata();
    let network = metadata.network;
    let steward_canister = metadata.steward_canister;
    let key_id = metadata.ecdsa_key_id;

    let wallet_key = SelfCustodyKey {
        network,
        owner: caller,
        steward_canister,
    };

    let raw_wallet = get_raw_wallet(&wallet_key);

    match raw_wallet {
        Some(wallet) => Ok(Wallet::from(wallet).address.to_string()),
        None => {
            let wallet = create_wallet(caller, steward_canister, network, key_id).await?;

            insert_wallet(wallet_key, wallet.clone().into());

            Ok(wallet.address.to_string())
        }
    }
}

fn insert_wallet(wallet_key: SelfCustodyKey, wallet: RawWallet) -> Option<RawWallet> {
    STATE.with(|s| s.borrow_mut().raw_wallet.insert(wallet_key, wallet))
}

async fn create_wallet(
    caller: Principal,
    steward_canister: Principal,
    network: BitcoinNetwork,
    key_id: EcdsaKeyId,
) -> Result<Wallet, WalletError> {
    base::utils::create_wallet(caller, steward_canister, network, key_id)
        .await
        .map_err(|e| WalletError::CreateWalletError(e.to_string()))
}
