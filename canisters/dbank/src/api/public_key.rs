use candid::Principal;
use wallet::domain::{AddressType, WalletType};

use crate::{
    domain::{Metadata, SelfCustodyKey},
    error::DBankError,
    repositories,
};

pub fn serve(wallet_owner: Principal, metadata: &Metadata) -> Result<Vec<u8>, DBankError> {
    let key1 = SelfCustodyKey::new(
        metadata.network,
        wallet_owner,
        metadata.steward_canister,
        WalletType::Single,
        AddressType::P2wpkh,
    );

    let key2 = SelfCustodyKey::new(
        metadata.network,
        wallet_owner,
        metadata.steward_canister,
        WalletType::Single,
        AddressType::P2pkh,
    );

    get_public_key(&key1)
        .or(get_public_key(&key2))
        .ok_or(DBankError::PublicKeyNotFound(wallet_owner.to_string()))
}

pub fn get_public_key(key: &SelfCustodyKey) -> Option<Vec<u8>> {
    repositories::wallet_info::get_info(key).map(|info| info.public_key)
}
