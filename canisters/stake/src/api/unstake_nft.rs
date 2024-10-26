use crate::domain::nft_types::{TransferResponse, User};
use candid::{Nat, Principal};
// use crate::domain::{StakeNFT, StakeStatus};
use crate::error::Error;
use crate::utils::get_token_hash;
use crate::{
    domain::{
        nft_types::{Service, TransferRequest},
        request::UnStakeRequest,
    },
    repositories,
};

pub async fn serve(unstake_request: UnStakeRequest) -> Result<bool, Error> {
    let stake_record = repositories::stake_nft::get_stake_record(
        unstake_request.nft_canister,
        unstake_request.nft_id,
    );
    if let Some(s) = stake_record {
        let token = get_token_hash(&unstake_request.nft_canister, &unstake_request.nft_id);
        let nft_service = Service(unstake_request.nft_canister);
        let redeem_result = redeem_nft(token, s.from, &nft_service).await;
        match redeem_result {
            Ok(b) => {
                if b {
                    let result = repositories::stake_nft::unstake_nft(
                        unstake_request.nft_canister,
                        unstake_request.nft_id,
                    );
                    result
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(e),
        }
    } else {
        Err(Error::NFTTUnstakeError("you do not stake nft".to_string()))
    }
    // Ok(true)
}

async fn redeem_nft(
    token: String,
    nft_owner: Principal,
    nft_service: &Service,
) -> Result<bool, Error> {
    let stake_canister = ic_cdk::id();
    let transfer_request = TransferRequest {
        to: User::Principal_(nft_owner),
        from: User::Principal_(stake_canister),
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
