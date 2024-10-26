
use crate::domain::StakeNFT;
use crate::repositories::stake_nft;

pub(crate) fn serve() -> Vec<StakeNFT> {
    stake_nft::get_all_stake_record()
}
