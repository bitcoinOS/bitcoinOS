use wallet::{
    domain::{AddressType, Wallet, WalletType},
    utils,
};

use crate::{
    context::STATE,
    domain::{Metadata, RawWallet, SelfCustodyKey},
    error::WalletError,
};

pub(crate) fn get_wallet(key: &SelfCustodyKey) -> Option<RawWallet> {
    STATE.with(|s| s.borrow().wallets.get(key).clone())
}

pub(crate) fn get_p2pkh_wallet(metadata: &Metadata) -> Option<RawWallet> {
    let key = SelfCustodyKey::new(metadata, WalletType::Single, AddressType::P2pkh);
    get_wallet(&key)
}

pub(crate) fn get_p2wpkh_wallet(metadata: &Metadata) -> Option<RawWallet> {
    let key = &SelfCustodyKey::new(metadata, WalletType::Single, AddressType::P2wpkh);
    get_wallet(key)
}

pub(crate) async fn get_or_create_p2pkh_wallet(metadata: Metadata) -> Result<Wallet, WalletError> {
    let raw_wallet = get_p2pkh_wallet(&metadata);

    match raw_wallet {
        Some(wallet) => Ok(Wallet::from(wallet)),
        None => {
            let wallet_key = SelfCustodyKey::new(&metadata, WalletType::Single, AddressType::P2pkh);

            let wallet =
                utils::create_p2pkh_wallet(metadata.owner, metadata.ecdsa_key_id, metadata.network)
                    .await?;

            insert_wallet(wallet_key, wallet.clone())?;

            Ok(wallet)
        }
    }
}

pub(crate) async fn get_or_create_p2wpkh_wallet(metadata: Metadata) -> Result<Wallet, WalletError> {
    let raw_wallet = get_p2wpkh_wallet(&metadata);

    match raw_wallet {
        Some(wallet) => Ok(Wallet::from(wallet)),
        None => {
            let wallet_key =
                SelfCustodyKey::new(&metadata, WalletType::Single, AddressType::P2wpkh);

            let wallet = utils::create_p2wpkh_wallet(
                metadata.owner,
                metadata.ecdsa_key_id,
                metadata.network,
            )
            .await?;

            insert_wallet(wallet_key, wallet.clone())?;

            Ok(wallet)
        }
    }
}

pub(crate) fn insert_wallet(wallet_key: SelfCustodyKey, wallet: Wallet) -> Result<(), WalletError> {
    STATE.with(|s| {
        let raw_wallet = &mut s.borrow_mut().wallets;

        match raw_wallet.get(&wallet_key) {
            Some(w) => Err(WalletError::WalletAlreadyExists(format!("{:?}", w))),
            None => {
                raw_wallet.insert(wallet_key, wallet.into());
                Ok(())
            }
        }
    })
}
