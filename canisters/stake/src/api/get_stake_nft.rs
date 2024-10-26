use candid::Principal;

use crate::domain::StakeNFT;
use crate::repositories::stake_nft;

pub(crate) fn serve(nft_canister: Principal, nft_id: u32) -> Option<StakeNFT> {
    stake_nft::get_stake_record(nft_canister, nft_id)
}
