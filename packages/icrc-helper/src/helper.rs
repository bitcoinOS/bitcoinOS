use std::f32::MIN;
use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;
use icrc_ledger_client::{TransferFromArgs, Account, Memo, Subaccount, TransferArg, NumTokens};
use crate::error::Error;

pub const MINT_ACCOUNT: Principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();


pub async fn mint(to: Principal, amount: Nat) -> Result<bool, Error> {
    let transfer_args = TransferFromArgs {
        spender_subaccount: None,
        from: Account {
            owner: MIN_ACCOUNT,
            subaccount: None,
        },
        to: Account {
            owner: to,
            subaccount: None,
        },
        amount,
        fee: None,
        memo: None,
        created_at_time: None,
    };

    match icrc_ledger_client::transfer_from(transfer_args).await {
        Ok(_) => { Ok(true) }
        Err(e) => Err(Error::TransferError(to, e.to_string())),
    }
}

pub async fn burn(from: Principal, amount: NumTokens) -> Result<Nat, Error> {
    let transfer_args = TransferArg {
        from_subaccount: None,
        to: Account {
            owner: MINT_ACCOUNT,
            subaccount: None,
        },
        fee: None,
        created_at_time: None,
        memo: None,
        amount,
    };

    match icrc_ledger_client::transfer(transfer_args).await {
        Ok(result) => Ok(result),
        Err(e) => Err(Error::TransferError(from, e.to_string())),
    }
}