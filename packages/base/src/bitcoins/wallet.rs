use sha2::Digest;

use ic_cdk::api::management_canister::bitcoin::{
    BitcoinNetwork, GetUtxosRequest, GetUtxosResponse, UtxoFilter,
};

use crate::{ecdsa, error::Error};

/// Returns the P2PKH address of this canister at the given derivation path.
pub async fn get_p2pkh_address(
    network: BitcoinNetwork,
    key_name: &str,
    derivation_path: Vec<Vec<u8>>,
) -> Result<String, Error> {
    let public_key = ecdsa::public_key(key_name, derivation_path, None).await?;

    Ok(public_key_to_p2pkh_address(network, &public_key))
}

/// Returns utxos for the given address, network and filter
pub async fn get_utxos(
    address: String,
    network: BitcoinNetwork,
    filter: Option<UtxoFilter>,
) -> Result<GetUtxosResponse, Error> {
    let req = GetUtxosRequest {
        address,
        network,
        filter,
    };
    ic_cdk::api::management_canister::bitcoin::bitcoin_get_utxos(req)
        .await
        .map(|(utxos,)| utxos)
        .map_err(|e| e.into())
}

fn public_key_to_p2pkh_address(network: BitcoinNetwork, public_key: &[u8]) -> String {
    // SHA-256 & RIPEMD-160
    let res = ripemd160(&sha256(public_key));

    let prefix = match network {
        BitcoinNetwork::Mainnet => 0x00,
        BitcoinNetwork::Testnet | BitcoinNetwork::Regtest => 0x6F,
    };

    let mut data_with_prefix = vec![prefix];
    data_with_prefix.extend(res);

    let checksum = &sha256(&sha256(&data_with_prefix.clone()))[..4];

    let mut full_address = data_with_prefix;
    full_address.extend(checksum);

    bs58::encode(full_address).into_string()
}

fn ripemd160(data: &[u8]) -> Vec<u8> {
    let mut hasher = ripemd::Ripemd160::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
