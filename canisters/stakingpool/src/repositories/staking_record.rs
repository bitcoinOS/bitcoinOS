use ic_cdk::api::management_canister::main::CanisterId;

use crate::{domain::TxID, error::StakingError};

/// Validate the sender is the staker and the amount is valid
pub fn validate_staker_amount(
    staker: CanisterId,
    amount_in_satoshi: u64,
    txid: &TxID,
) -> Result<(), StakingError> {
    todo!()
}
