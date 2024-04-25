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

use super::{get_raw_wallet, insert_wallet};

pub async fn serve(owner: Principal, metadata: Metadata) -> Result<String, WalletError> {
    let wallet_key = SelfCustodyKey::new(
        owner,
        &metadata,
        base::domain::WalletType::Single,
        AddressType::P2wsh,
    );

    let raw_wallet = get_raw_wallet(&wallet_key);

    match raw_wallet {
        Some(wallet) => Ok(Wallet::from(wallet).address.to_string()),
        None => {
            let wallet =
                create_single_p2wsh_wallet(owner, metadata.ecdsa_key_id, metadata.network).await?;

            let address = wallet.address.to_string();

            insert_wallet(wallet_key, wallet)?;

            Ok(address)
        }
    }
}

async fn create_single_p2wsh_wallet(
    owner: Principal,
    key_id: EcdsaKeyId,
    network: BitcoinNetwork,
) -> Result<Wallet, WalletError> {
    let derivation_path = principal_to_derivation_path(owner);
    let public_key = base::ecdsa::public_key(derivation_path.clone(), key_id, None).await?;

    base::bitcoins::create_p2wsh_wallet(derivation_path, &public_key, network)
        .await
        .map_err(|e| e.into())
}
