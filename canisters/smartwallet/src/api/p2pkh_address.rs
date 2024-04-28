use base::{
    domain::{AddressType, Wallet},
    utils::principal_to_derivation_path,
};
use candid::Principal;
use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, ecdsa::EcdsaKeyId};

use crate::{
    domain::{Metadata, SelfCustodyKey},
    error::WalletError,
};

use super::get_raw_wallet;

/// Returns the P2PKH address of this canister
/// if P2PKH address not exist, create a new one and and save it to stable storage
pub(super) async fn serve(owner: Principal, metadata: Metadata) -> Result<String, WalletError> {
    get_or_create_p2pkh_wallet(owner, metadata)
        .await
        .map(|w| w.address.to_string())
}

async fn create_p2pkh_wallet(
    owner: Principal,
    key_id: EcdsaKeyId,
    network: BitcoinNetwork,
) -> Result<Wallet, WalletError> {
    let derivation_path = principal_to_derivation_path(owner);
    let public_key = base::ecdsa::public_key(derivation_path.clone(), key_id, None).await?;

    base::bitcoins::create_p2pkh_wallet(derivation_path, &public_key, network)
        .await
        .map_err(|e| e.into())
}

pub async fn get_or_create_p2pkh_wallet(
    owner: Principal,
    metadata: Metadata,
) -> Result<Wallet, WalletError> {
    let wallet_key = SelfCustodyKey::new(
        owner,
        &metadata,
        base::domain::WalletType::Single,
        AddressType::P2pkh,
    );

    let raw_wallet = get_raw_wallet(&wallet_key);

    match raw_wallet {
        Some(wallet) => Ok(Wallet::from(wallet)),
        None => {
            let wallet =
                create_p2pkh_wallet(owner, metadata.ecdsa_key_id, metadata.network).await?;

            super::insert_wallet(wallet_key, wallet.clone())?;

            Ok(wallet)
        }
    }
}
