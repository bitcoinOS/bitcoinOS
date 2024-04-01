use base::ICBitcoinNetwork;
use candid::Principal;

use crate::{
    context::STATE,
    domain::{RawWallet, SelfCustodyKey, Wallet},
    error::WalletError,
};

/// Get an exist address, or generate a new address by caller
/// Returns a address if success, or returns `CreateWalletFailed`
/// TODO: support multiple addresses
pub async fn get_or_create_wallet_address(caller: Principal) -> Result<String, WalletError> {
    let (raw_wallet, wallet_key, network, steward_canister, key_name) =
        get_wallet_key_network_steward(caller);

    match raw_wallet {
        Some(wallet) => Ok(Wallet::from(wallet).address.to_string()),
        None => {
            let wallet = create_wallet(caller, steward_canister, network, key_name).await?;

            insert_wallet(wallet_key, wallet.clone().into());

            Ok(wallet.address.to_string())
        }
    }
}

fn get_wallet_key_network_steward(
    caller: Principal,
) -> (
    Option<RawWallet>,
    SelfCustodyKey,
    ICBitcoinNetwork,
    Principal,
    String,
) {
    STATE.with(|s| {
        let state = s.borrow();
        let metadata = state.metadata.get();

        let network = metadata.network;
        let steward_canister = metadata.steward_canister;
        let key_name = metadata.key_name.clone();
        let wallet_key = SelfCustodyKey {
            network,
            owner: caller,
            steward_canister,
        };
        let wallet = state.raw_wallet.get(&wallet_key);
        (wallet, wallet_key, network, steward_canister, key_name)
    })
}

fn insert_wallet(wallet_key: SelfCustodyKey, wallet: RawWallet) -> Option<RawWallet> {
    STATE.with(|s| s.borrow_mut().raw_wallet.insert(wallet_key, wallet))
}

async fn create_wallet(
    caller: Principal,
    steward_canister: Principal,
    network: ICBitcoinNetwork,
    key_name: String,
) -> Result<Wallet, WalletError> {
    base::utils::create_wallet(caller, steward_canister, network, key_name)
        .await
        .map_err(|e| WalletError::CreateWalletError(e.to_string()))
}
