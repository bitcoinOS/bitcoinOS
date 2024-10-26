use bitcoin::Amount;

use ic_cdk::api::management_canister::main::CanisterId;

use wallet::tx::RecipientAmount;

use crate::domain::request::RedeemRequest;
use crate::domain::Metadata;
use crate::repositories;
use crate::repositories::counter;
use crate::repositories::tx_log;
use wallet::error::StakingError;

use super::transfer_from_p2wsh_multisig22;

pub(super) async fn serve(
    sender: CanisterId,
    metadata: Metadata,
    req: RedeemRequest,
    redeem_time: u64,
) -> Result<String, StakingError> {
    let txid = req.txid.clone();

    let amount = repositories::staking_record::validate_staker_amount(sender, &txid, redeem_time)?;

    let recipient = req.validate_address()?;
    let tx = RecipientAmount {
        recipient,
        amount: Amount::from_sat(amount),
    };

    // Log transfer info
    tx_log::build_and_append_redeem_log(req)?;

    // Transaction counter increment one
    counter::increment_one();

    // Update the staking record status as `Redeeming`
    repositories::staking_record::redeeming_record(txid.clone(), redeem_time)?;

    let redeemed_txid = transfer_from_p2wsh_multisig22::serve(metadata, &[tx]).await?;

    repositories::staking_record::redeemed_record(txid, redeem_time, redeemed_txid.clone())?;

    ic_cdk::print(format!("Redeemed tx is {redeemed_txid:?} \n"));

    Ok(redeemed_txid)
}
