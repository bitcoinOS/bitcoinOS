use candid::Principal;

use wallet::{
    domain::{AddressType, Wallet, WalletType},
    utils,
};

use crate::{
    domain::{Metadata, SelfCustodyKey},
    error::WalletError,
};

use super::get_wallet;

/// Returns the P2PKH address of this canister
/// if P2PKH address not exist, create a new one and and save it to stable storage
pub(super) async fn serve(owner: Principal, metadata: Metadata) -> Result<String, WalletError> {
    get_or_create_p2pkh_wallet(owner, metadata)
        .await
        .map(|w| w.address.to_string())
}

pub async fn get_or_create_p2pkh_wallet(
    owner: Principal,
    metadata: Metadata,
) -> Result<Wallet, WalletError> {
    let wallet_key = SelfCustodyKey::new(owner, &metadata, WalletType::Single, AddressType::P2pkh);

    let raw_wallet = get_wallet(&wallet_key);

    match raw_wallet {
        Some(wallet) => Ok(Wallet::from(wallet)),
        None => {
            let wallet =
                utils::create_p2pkh_wallet(owner, metadata.ecdsa_key_id, metadata.network).await?;

            super::insert_wallet(wallet_key, wallet.clone())?;

            Ok(wallet)
        }
    }
}
