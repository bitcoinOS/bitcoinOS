use base::domain::{AddressType, Wallet};
use candid::Principal;
use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, ecdsa::EcdsaKeyId};

use crate::{
    domain::{Metadata, SelfCustodyKey},
    error::WalletError,
};

use super::get_raw_wallet;

/// Get an exist address, or generate a new address by caller
/// Returns a address if success, or returns `CreateWalletFailed`
/// TODO: support multiple addresses for different diravation path
pub async fn serve(owner: Principal, metadata: Metadata) -> Result<String, WalletError> {
    let wallet_key = SelfCustodyKey::new(
        owner,
        &metadata,
        base::domain::WalletType::MultiSig22,
        AddressType::P2wsh,
    );

    let raw_wallet = get_raw_wallet(&wallet_key);

    match raw_wallet {
        Some(wallet) => Ok(Wallet::from(wallet).address.to_string()),
        None => {
            let wallet = create_multisig22_wallet(
                owner,
                metadata.steward_canister,
                metadata.network,
                metadata.ecdsa_key_id,
            )
            .await?;
            let address = wallet.address.to_string();

            super::insert_wallet(wallet_key, wallet)?;

            Ok(address)
        }
    }
}

async fn create_multisig22_wallet(
    caller: Principal,
    steward_canister: Principal,
    network: BitcoinNetwork,
    key_id: EcdsaKeyId,
) -> Result<Wallet, WalletError> {
    base::utils::create_multisig22_wallet(caller, steward_canister, network, key_id)
        .await
        .map_err(|e| WalletError::CreateWalletError(e.to_string()))
}
