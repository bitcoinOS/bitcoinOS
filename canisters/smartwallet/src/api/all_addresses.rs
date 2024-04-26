use candid::Principal;

use crate::domain::{Metadata, SelfCustodyKey};

use super::get_raw_wallet;

/// Returns all addresses of this canister
/// There're three types address(p2pkh, p2wsh, p2wsh multisig2-2)
pub(super) async fn serve(owner: Principal, metadata: Metadata) -> Vec<String> {
    let mut addresses = vec![];

    let p2pkh_key = SelfCustodyKey::new(
        owner,
        &metadata,
        base::domain::WalletType::Single,
        base::domain::AddressType::P2pkh,
    );

    let p2pkh_wallet = get_raw_wallet(&p2pkh_key);

    if let Some(w) = p2pkh_wallet {
        addresses.push(w.address);
    }

    let p2wsh_key = SelfCustodyKey::new(
        owner,
        &metadata,
        base::domain::WalletType::Single,
        base::domain::AddressType::P2wsh,
    );

    let p2wsh_wallet = get_raw_wallet(&p2wsh_key);

    if let Some(w) = p2wsh_wallet {
        addresses.push(w.address);
    }

    let p2pwsh_multisig22_key = SelfCustodyKey::new(
        owner,
        &metadata,
        base::domain::WalletType::MultiSig22,
        base::domain::AddressType::P2wsh,
    );

    let p2wsh_multisig22_wallet = get_raw_wallet(&p2pwsh_multisig22_key);

    if let Some(w) = p2wsh_multisig22_wallet {
        addresses.push(w.address);
    }

    addresses
}
