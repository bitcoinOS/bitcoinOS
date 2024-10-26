use candid::{Nat, Principal};
use wallet::utils::ic_time;

use crate::domain::{StakeNFT, StakeStatus};
use crate::error::Error;
use crate::{
    domain::{
        nft_types::{
            AllowanceRequest, BalanceRequest, BalanceResponse, Result12, Service, TransferRequest,
            TransferResponse, User,
        },
        request::StakeRequest,
    },
    repositories,
    utils::get_token_hash,
};

pub async fn serve(stake_request: StakeRequest) -> Result<bool, Error> {
    let nft_service = Service(stake_request.nft_canister);
    let stake_canister = ic_cdk::id();
    let caller = ic_cdk::caller();
    //owner 是否绑定到caller
    let bind_result = check_user_bind_wallet(caller, stake_request.nft_owner.to_text()).await;
    match bind_result {
        Ok(b) => {
            if !b {
                return Err(Error::WalletBindError("you do not bind wallet".to_string()));
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    //查询nft owner
    let token = get_token_hash(&stake_request.nft_canister, &stake_request.nft_id);
    let owner_status = check_nft_owner(token.clone(), stake_request.nft_owner, &nft_service).await;
    match owner_status {
        Ok(b) => {
            if !b {
                return Err(Error::NFTBalanceError("you do not ownd nft".to_string()));
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    //nft 是否授权给合约
    let allowance_status = check_nft_allowance(
        token.clone(),
        stake_request.nft_owner,
        stake_canister,
        &nft_service,
    )
    .await;
    match allowance_status {
        Ok(b) => {
            if !b {
                return Err(Error::NFTAllownceError(
                    "you do not approve nft".to_string(),
                ));
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    //transfer
    let transfer_status = transfer_nft(token.clone(), stake_request.nft_owner, &nft_service).await;
    match transfer_status {
        Ok(b) => {
            if !b {
                return Err(Error::NFTAllownceError(
                    "you do not approve nft".to_string(),
                ));
            } else {
                let stake_nft = StakeNFT {
                    nft_canister: stake_request.nft_canister,
                    from: stake_request.nft_owner,
                    staker: caller,
                    nft_id: stake_request.nft_id,
                    to: stake_canister,
                    amount: 1,
                    stake_status: StakeStatus::Stake,
                    stake_at: ic_time(),
                    unstake_at: 0,
                };
                let _ = repositories::stake_nft::stake_nft(stake_nft);
                return Ok(true);
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
}

async fn check_nft_owner(
    token: String,
    owner: Principal,
    nft_service: &Service,
) -> Result<bool, Error> {
    let balance_request = BalanceRequest {
        token: token.clone(),
        user: User::Principal_(owner),
    };
    let staker_balance_resp: Result<(BalanceResponse,), _> =
        nft_service.balance(balance_request).await;
    let nft_balance = staker_balance_resp
        .map(|(a,)| a)
        .map_err(|(_, msg)| Error::NFTBalanceError(msg));
    match nft_balance {
        Ok(br) => {
            match br {
                BalanceResponse::Ok(b) => {
                    //拥有nft
                    if b.eq(&1u8) {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
                BalanceResponse::Err(e) => Err(Error::NFTBalanceError(format!(
                    "1:check_nft_owner:get nft balance error,{owner},{token}"
                ))),
            }
        }
        Err(e) => Err(Error::NFTBalanceError(format!(
            "2:check_nft_owner:get nft balance error,{owner},{token},err:{}",
            e.to_string()
        ))),
    }
}

async fn check_nft_allowance(
    token: String,
    owner: Principal,
    spender: Principal,
    service: &Service,
) -> Result<bool, Error> {
    let allowance_request = AllowanceRequest {
        token: token.clone(),
        owner: User::Principal_(owner),
        spender: spender,
    };
    let allowance_balance_resp: Result<(Result12,), _> = service.allowance(allowance_request).await;
    let allownce_balance = allowance_balance_resp
        .map(|(a,)| a)
        .map_err(|(_, msg)| Error::NFTAllownceError(msg));
    match allownce_balance {
        Ok(br) => {
            match br {
                Result12::Ok(b) => {
                    //拥有nft
                    if b.eq(&1u8) {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
                Result12::Err(e) => Err(Error::NFTAllownceError(format!(
                    "1:get nft allowance error,{owner},{token},{spender}"
                ))),
            }
        }
        Err(e) => Err(Error::NFTAllownceError(format!(
            "2:get nft allowance error,{owner},{token},{spender}:{}",
            e.to_string()
        ))),
    }
}

async fn check_user_bind_wallet(user: Principal, wallet_address: String) -> Result<bool, Error> {
    let metadata = repositories::metadata::get_metadata();
    let user_canister = metadata.user_canister;
    let bind_result: Result<(bool,), _> = ic_cdk::call(
        user_canister,
        "check_wallet_bind_status",
        (user, wallet_address),
    )
    .await;
    let bind_wallet_info = bind_result
        .map(|(a,)| a)
        .map_err(|(_, msg)| Error::WalletBindError(msg));
    bind_wallet_info
}

async fn transfer_nft(
    token: String,
    nft_owner: Principal,
    nft_service: &Service,
) -> Result<bool, Error> {
    let stake_canister = ic_cdk::id();
    let transfer_request = TransferRequest {
        to: User::Principal_(stake_canister),
        from: User::Principal_(nft_owner),
        token: token,
        notify: false,
        memo: serde_bytes::ByteBuf::from(vec![0]),
        amount: Nat::from(1u8),
        subaccount: None,
    };
    let transfer_resp: Result<(TransferResponse,), _> =
        nft_service.transfer(transfer_request).await;
    let transfer_info = transfer_resp
        .map(|(a,)| a)
        .map_err(|(_, msg)| Error::NFTTransferError(msg));
    match transfer_info {
        Ok(tr) => match tr {
            TransferResponse::Ok(b) => {
                if b.eq(&1u8) {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            TransferResponse::Err(e) => {
                Err(Error::NFTTransferError("transfer  nft error".to_string()))
            }
        },
        Err(e) => Err(Error::NFTTransferError(e.to_string())),
    }
}
