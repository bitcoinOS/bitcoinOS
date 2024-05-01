use crate::domain::StakingPoolInfo;

pub(crate) fn serve() -> Vec<StakingPoolInfo> {
    crate::repositories::staking_pool::list_staking_pool()
}
