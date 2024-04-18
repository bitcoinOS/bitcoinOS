use base::domain::Wallet;
use candid::Principal;
use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, ecdsa::EcdsaKeyId};

use crate::{domain::SelfCustodyKey, error::WalletError};

use super::{get_raw_wallet, insert_wallet};

pub async fn serve(
    caller: Principal,
    key_id: EcdsaKeyId,
    steward_canister: Principal,
    network: BitcoinNetwork,
) -> Result<String, WalletError> {
    let wallet_key = SelfCustodyKey {
        network,
        owner: caller,
        steward_canister,
    };

    let raw_wallet = get_raw_wallet(&wallet_key);

    match raw_wallet {
        Some(wallet) => Ok(Wallet::from(wallet).address.to_string()),
        None => {
            let wallet = create_single_p2wsh_wallet(caller, key_id, network).await?;

            let address = wallet.address.to_string();

            insert_wallet(wallet_key, wallet);

            Ok(address)
        }
    }
}

async fn create_single_p2wsh_wallet(
    caller: Principal,
    key_id: EcdsaKeyId,
    network: BitcoinNetwork,
) -> Result<Wallet, WalletError> {
    let derivation_path = vec![caller.as_slice().to_vec()];
    let public_key = base::ecdsa::public_key(derivation_path.clone(), key_id, None).await?;

    base::bitcoins::create_p2wsh_wallet(derivation_path, &public_key, network)
        .await
        .map_err(|e| e.into())
}
