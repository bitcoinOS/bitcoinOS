use candid::Principal;

use wallet::{
    bitcoins,
    domain::{AddressType, Wallet, WalletType},
    utils::{self, ic_time, principal_to_derivation_path},
};

use crate::{
    context::STATE,
    domain::{
        CreateWalletEvent, DBankWalletInfo, Metadata, RawWallet, SelfCustodyKey,
        WalletOperationEvent,
    },
    error::DBankError,
};

pub(crate) fn get_wallet(key: &SelfCustodyKey) -> Option<RawWallet> {
    STATE.with(|s| s.borrow().wallets.get(key).clone())
}

pub(crate) fn get_p2pkh_wallet(metadata: &Metadata, wallet_owner: Principal) -> Option<RawWallet> {
    let key = SelfCustodyKey::new(
        metadata.network,
        wallet_owner,
        metadata.steward_canister,
        WalletType::Single,
        AddressType::P2pkh,
    );
    get_wallet(&key)
}

pub(crate) fn get_p2wpkh_wallet(metadata: &Metadata, wallet_owner: Principal) -> Option<RawWallet> {
    let key = &SelfCustodyKey::new(
        metadata.network,
        wallet_owner,
        metadata.steward_canister,
        WalletType::Single,
        AddressType::P2wpkh,
    );
    get_wallet(key)
}

pub(crate) async fn get_or_create_p2pkh_wallet(
    seq_in_os: u64,
    metadata: Metadata,
    wallet_owner: Principal,
    name: String,
) -> Result<Wallet, DBankError> {
    let raw_wallet = get_p2pkh_wallet(&metadata, wallet_owner);
    let key_id = metadata.ecdsa_key_id.clone();
    let network = metadata.network;

    match raw_wallet {
        Some(wallet) => Ok(Wallet::from(wallet)),
        None => {
            let wallet_key = SelfCustodyKey::new(
                metadata.network,
                wallet_owner,
                metadata.steward_canister,
                WalletType::Single,
                AddressType::P2pkh,
            );

            let wallet = utils::create_p2pkh_wallet(wallet_owner, key_id.clone(), network).await?;

            let wallet_info = build_wallet_info(
                &metadata,
                wallet_owner,
                wallet.address.to_string(),
                seq_in_os,
                name,
                AddressType::P2pkh,
                WalletType::Single,
            )
            .await?;

            insert_wallet(wallet_key, wallet.clone(), &wallet_info)?;

            Ok(wallet)
        }
    }
}

pub(crate) async fn get_or_create_p2wpkh_wallet(
    seq_in_os: u64,
    metadata: Metadata,
    wallet_owner: Principal,
    name: String,
) -> Result<Wallet, DBankError> {
    let raw_wallet = get_p2wpkh_wallet(&metadata, wallet_owner);

    match raw_wallet {
        Some(wallet) => Ok(Wallet::from(wallet)),
        None => {
            let wallet_key = SelfCustodyKey::new(
                metadata.network,
                wallet_owner,
                metadata.steward_canister,
                WalletType::Single,
                AddressType::P2wpkh,
            );

            let wallet = bitcoins::create_p2wpkh_wallet(
                wallet_owner,
                metadata.ecdsa_key_id.clone(),
                metadata.network,
            )
            .await?;

            let wallet_info = build_wallet_info(
                &metadata,
                wallet_owner,
                wallet.address.to_string(),
                seq_in_os,
                name,
                AddressType::P2wpkh,
                WalletType::Single,
            )
            .await?;

            insert_wallet(wallet_key, wallet.clone(), &wallet_info)?;

            Ok(wallet)
        }
    }
}

pub(crate) fn insert_wallet(
    wallet_key: SelfCustodyKey,
    wallet: Wallet,
    wallet_info: &DBankWalletInfo,
) -> Result<(), DBankError> {
    STATE.with(|s| {
        let state = &mut s.borrow_mut();
        let raw_wallet = &mut state.wallets;

        match raw_wallet.get(&wallet_key) {
            Some(w) => Err(DBankError::WalletAlreadyExists(format!("{:?}", w))),
            None => {
                raw_wallet.insert(wallet_key, wallet.into());

                let wallet_infos = &mut state.wallet_infos;
                wallet_infos.insert(wallet_key, wallet_info.to_owned());

                let wallet_logs = &mut state.wallet_logs;
                wallet_logs
                    .append(&WalletOperationEvent::CreateWallet(CreateWalletEvent {
                        wallet_info: wallet_info.clone(),
                    }))
                    .map_err(|e| DBankError::AppendTransferLogError(format!("{:?}", e)))?;

                Ok(())
            }
        }
    })
}

pub(crate) async fn build_wallet_info(
    metadata: &Metadata,
    wallet_owner: Principal,
    bitcoin_address: String,
    seq_in_os: u64,
    name: String,
    address_type: AddressType,
    wallet_type: WalletType,
) -> Result<DBankWalletInfo, DBankError> {
    // Increment seq_in_dbank
    let seq_in_bank = super::sequencer::increment_one();
    // let bitcoin_address = wallet.address.to_string();
    let derivation_path = principal_to_derivation_path(wallet_owner);

    let public_key =
        wallet::ecdsa::public_key(derivation_path, metadata.ecdsa_key_id.clone(), None).await?;

    let wallet_info = DBankWalletInfo {
        seq_in_os,
        seq_in_bank,
        owner: wallet_owner,
        name,
        bitcoin_address,
        public_key,
        network: metadata.network,
        address_type,
        wallet_type,
        dbank_canister: metadata.dbank_canister,
        steward_canister: metadata.steward_canister,
        created_at: ic_time(),
        status: Default::default(),
    };

    Ok(wallet_info)
}
