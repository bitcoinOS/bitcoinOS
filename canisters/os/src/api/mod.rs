mod add_controller;
mod append_wallet_action;
// mod confirm_staking_record;
mod count_wallet;
mod create_dbank_canister;
mod create_staking_pool;
mod current_dbank;
mod get_canister_module;
mod get_staking_record;
mod get_user_info;
mod get_wallet_action;
mod install_staking_pool_wasm;
mod list_dbank;
mod list_staking_pool;
mod list_staking_record;
mod list_wallet;
mod list_wallet_types;
mod login_or_create_user;
mod my_wallet;
mod register_canister_module;
// mod register_staking_record;
mod registry_dbank;
mod registry_staking_pool;
mod registry_wallet;

mod set_wallet_cycles;
mod staking_pool_increment_one;
mod tvl;
mod tvl_of;
mod update_invited_code;
mod update_last_reward;
mod update_staking_pool_bitcoin_address;
mod update_staking_pool_info;
mod update_staking_pool_status;
mod update_wallet_bitcoin_address;
mod upgrade_dbank_wasm;
mod upgrade_staking_pool_wasm;

mod bind_wallet;
mod check_wallet_bind_user;
mod get_bind_wallet;
mod get_user_count;
mod unbind_wallet;
mod update_user_profile;
mod wallet_counter_increment_one;

use crate::{
    constants::{self, DBANK_WASM, DEFAULT_CYCLES_PER_DBANK},
    domain::{
        request::{BindRequest, UserProfileRequest, UserRequest},
        CanisterModuleInfo, DBankInfo, UserInfo, UserStatus, WalletBindInfo,
    },
    repositories::{user::set_user_invite_code, wallet_bind::check_wallet_bind_user},
};
use candid::Principal;

use ic_cdk::{
    api::{
        is_controller,
        management_canister::{bitcoin::Satoshi, main::CanisterId},
    },
    export_candid, init,
};
use wallet::{
    constants::{MAX_WALLET_LIMIT, START_ID_IN_DBANK_ID},
    domain::{
        request::{
            CreateDBankWalletRequest, UpdateStakingPoolInfoRequest, UpdateStakingPoolStatusRequest,
        },
        response::UpdateStakingPoolInfoResponse,
        reward::{Reward, RewardStaus, RewardType},
        staking::{InitStakingPoolArgument, StakingPoolInfo, StakingRecord},
        user::UserType,
    },
    error::StakingError,
    utils::{check_normal_principal, ic_caller, ic_time},
};

use crate::{
    constants::{DEFAULT_CYCLES_PER_CANISTER, MAX_WALLET_PER_USER, STAKING_POOL_WASM},
    context::STATE,
    domain::{
        request::{
            CreateStakingPoolRequest, InitArgument, RegisterStakingPoolRequest,
            UpdateBitcoinAddressRequest,
        },
        Action, Metadata, WalletAction, WalletInfo,
    },
    error::Error,
    repositories::{self, staking_pool_counter},
};

/// ---------------- Update interface of this canister ------------------
///
/// Create a smart wallet canister, log the action, and store the wallet owner info
#[ic_cdk::query]
async fn create_wallet_canister(name: String) -> Result<Principal, Error> {
    // Replace with `DBank caniter` to create wallet
    Err(Error::UnAuthorized(name.to_string()))
}


/// Update Staking pool bitcoin address if neccessary
#[ic_cdk::update]
fn update_wallet_bitcoin_address(req: UpdateBitcoinAddressRequest) -> Result<WalletInfo, Error> {
    if is_controller(&ic_cdk::caller()) {
        update_wallet_bitcoin_address::serve(req.canister_id, req.bitcoin_address)
    } else {
        Err(Error::UnAuthorized(ic_cdk::caller().to_string()))
    }
}

/// Create a Staking Pool with given annualized interest rate and duration, name and description
#[ic_cdk::update]
async fn create_staking_pool_canister(
    req: CreateStakingPoolRequest,
) -> Result<StakingPoolInfo, Error> {
    let owner = ic_cdk::caller();

    if !is_controller(&owner) {
        return Err(Error::UnAuthorized(owner.to_string()));
    }

    let os_canister = ic_cdk::id();
    let created_at = ic_cdk::api::time();
    let metadata = get_metadata();

    let init_arg: InitStakingPoolArgument = InitStakingPoolArgument {
        name: req.name,
        description: req.description,
        network: metadata.network,
        annual_interest_rate: req.annual_interest_rate,
        duration_in_day: req.duration_in_day,
        os_canister,
        steward_canister: metadata.steward_canister,
        status: req.status,
        start_time: req.start_time,
        // stake_end_time: req.stake_end_time,
        end_time: req.end_time,
        fund_management: req.fund_management,
        boost_rate: req.boost_rate,
        minimum_stake_amount: req.minimum_stake_amount,
    };
  
    let point_canister_option = get_canister_module::serve("point".to_string()).await;
    if let Some(point_canister) = point_canister_option {
        let staking_pool_canister = create_staking_pool::serve(
            init_arg.clone(),
            STAKING_POOL_WASM.to_owned(),
            metadata.wallet_cycles,
            vec![os_canister, owner, point_canister.canister_id],
        )
        .await
        .map_err(|msg| Error::CreateCanisterFailed { msg })?;

        ic_cdk::print("Created staking pool canister ----------- \n");

        let staking_pool_address = fetch_wallet_address(staking_pool_canister).await?;

        let info = registry_staking_pool::serve(
            staking_pool_canister,
            created_at,
            staking_pool_address,
            init_arg,
        )?;

        staking_pool_increment_one::serve()?;

        Ok(info)
    } else {
        Err(Error::CanisterNotExists {
            canister_name: "point".to_string(),
        })
    }
}

/// Register a new staking pool canister if create new staking pool failed before
#[ic_cdk::update]
fn register_staking_pool(req: RegisterStakingPoolRequest) -> Result<StakingPoolInfo, Error> {
    let owner = ic_cdk::caller();

    if !is_controller(&owner) {
        return Err(Error::UnAuthorized(owner.to_string()));
    }

    let os_canister = ic_cdk::id();
    let created_at = ic_cdk::api::time();
    let metadata = get_metadata();

    ic_cdk::print("Created staking pool canister ----------- \n");

    // let staking_pool_address = fetch_wallet_address(staking_pool_id).await?;

    let init_arg = InitStakingPoolArgument {
        name: req.name,
        description: req.description,
        network: metadata.network,
        annual_interest_rate: req.annual_interest_rate,
        duration_in_day: req.duration_in_day,
        os_canister,
        steward_canister: metadata.steward_canister,
        status: req.status,
        start_time: req.start_time,
        // stake_end_time: req.stake_end_time,
        end_time: req.end_time,
        fund_management: req.fund_management,
        boost_rate: req.boost_rate,
        minimum_stake_amount: req.minimum_stake_amount,
    };

    let info = registry_staking_pool::serve(
        req.staking_pool_canister,
        created_at,
        req.bitcoin_address,
        init_arg,
    )?;

    staking_pool_increment_one::serve()?;

    Ok(info)
}

/// Update staking pool with new wasm file for tests
/// TODO: Remove this once tests when deploy to mainnet
#[ic_cdk::update]
async fn install_staking_pool_wasm(
    staking_pool_canister: CanisterId,
    reinstall: bool,
) -> Result<(), String> {
    if is_controller(&ic_cdk::caller()) {
        install_staking_pool_wasm::serve(
            staking_pool_canister,
            STAKING_POOL_WASM.to_owned(),
            reinstall,
        )
        .await
    } else {
        Err("UnAuthorized".to_string())
    }
}

/// Update Staking pool bitcoin address if neccessary
#[ic_cdk::update]
fn update_staking_pool_bitcoin_address(
    req: UpdateBitcoinAddressRequest,
) -> Result<StakingPoolInfo, Error> {
    if is_controller(&ic_cdk::caller()) {
        update_staking_pool_bitcoin_address::serve(req.canister_id, req.bitcoin_address)
    } else {
        Err(Error::UnAuthorized(ic_cdk::caller().to_string()))
    }
}

/// Update staking pool with new wasm file for tests
/// TODO: Remove this once tests when deploy to mainnet
#[ic_cdk::update]
async fn upgrade_staking_pool_wasm(staking_pool_canister: CanisterId) -> Result<(), String> {
    if is_controller(&ic_cdk::caller()) {
        upgrade_staking_pool_wasm::serve(staking_pool_canister, STAKING_POOL_WASM.to_owned()).await
    } else {
        Err("UnAuthorized".to_string())
    }
}

/// Update staking pool with new wasm file for tests
/// TODO: Remove this once tests when deploy to mainnet
#[ic_cdk::update]
async fn upgrade_dbank_wasm(dbank_id: u64, dbank_canister: CanisterId) -> Result<(), String> {
    if is_controller(&ic_cdk::caller()) {
        upgrade_dbank_wasm::serve(dbank_id, dbank_canister, DBANK_WASM.to_owned()).await
    } else {
        Err("UnAuthorized".to_string())
    }
}

/// Update staking pool metadata info
#[ic_cdk::update]
async fn update_staking_pool_info(
    req: UpdateStakingPoolInfoRequest,
) -> UpdateStakingPoolInfoResponse {
    let owner = ic_caller();
    validate_owner(owner).expect("caller must be controller or owner");

    update_staking_pool_info::serve(req)
        .await
        .expect("failed to update staking pool info")
}

/// Update staking pool status to the given value
#[ic_cdk::update]
async fn update_staking_pool_status(req: UpdateStakingPoolStatusRequest) -> String {
    let owner = ic_caller();
    validate_owner(owner).expect("caller must be controller or owner");

    update_staking_pool_status::serve(req)
        .await
        .expect("failed to update staking pool status")
}

/// Update the default cycles for a wallet canister when creating a wallet
/// NOTE: Only controller can update
#[ic_cdk::update]
fn set_wallet_cycles(wallet_cycles: u64) -> Result<u64, Error> {
    if is_controller(&ic_cdk::caller()) {
        set_wallet_cycles::serve(wallet_cycles)
    } else {
        Err(Error::UnAuthorized(ic_cdk::caller().to_string()))
    }
}

/// Add new controller to a canister
#[ic_cdk::update]
async fn add_controller(
    canister_id: CanisterId,
    controllers: Vec<Principal>,
) -> Result<bool, Error> {
    let caller = ic_cdk::caller();
    let os_canister = ic_cdk::id();

    let mut controllers = controllers;
    controllers.extend(vec![caller, os_canister]);

    if is_controller(&ic_cdk::caller()) {
        add_controller::serve(canister_id, controllers).await
    } else {
        Err(Error::UnAuthorized(ic_cdk::caller().to_string()))
    }
}


#[ic_cdk::update]
async fn register_canister_module(
    canister_name: String,
    canister_id: CanisterId,
) -> Result<bool, Error> {
    let caller = ic_cdk::caller();
    if !is_controller(&caller) {
        return Err(Error::UnAuthorized(caller.to_string()));
    }
    register_canister_module::serve(canister_name, canister_id).await
}

async fn init_user(user_id: Principal, user_type: UserType) {
    // ic_cdk::print("init user -1");
    let point = repositories::canister_module::get_canister_module("point".to_string());
    if let Some(p) = point {
        // ic_cdk::print("init user 1");
        let _: Result<(bool,), _> =
            ic_cdk::call(p.canister_id, "init_user", ((user_id, user_type),)).await;
        // ic_cdk::print("init user 2");
    };
}

#[ic_cdk::update]
async fn login_or_create(user_request: UserRequest) -> Result<UserInfo, Error> {
    let param_res = check_user_params(user_request.clone()).await;
    // let param_res = Ok(true);

    match param_res {
        Ok(_) => {
            init_user(user_request.user_id, user_request.user_type).await;
            let user = get_user_info(user_request.user_id);

            let primary_wallet = if user_request.user_type == UserType::II {
                get_or_create_my_wallet(user_request.name.clone())
                    .await
                    .ok()
            } else {
                None
            };

            match user {
                // login
                Some(u) => {
                    // Comment for ii canister wallet
                    let wallet_canister = if u.primary_wallet.is_none() {
                        primary_wallet
                    } else {
                        u.primary_wallet
                    };

                    let new_user = UserInfo {
                        primary_wallet: wallet_canister,
                        ..u
                    };

                    login_or_create_user::serve(new_user)
                }
                //create
                None => {
                    let metadata = get_metadata();
                    let seq = repositories::user::get_user_seq().unwrap();
                    let invite_code = wallet::utils::gen_invite_code(seq);
                    let invite_code_res =
                        set_user_invite_code(invite_code.clone(), user_request.user_id);

                    match invite_code_res {
                        Ok(_) => {
                            let t = wallet::utils::ic_time();
                            let new_user = UserInfo {
                                seq,
                                name: user_request.name,
                                user_id: user_request.user_id,
                                user_desc: user_request.user_desc,
                                user_img: user_request.user_img,
                                user_type: user_request.user_type,
                                wallet_address: user_request.wallet_address,
                                // Comment for ii canister wallet 2024-08-13
                                // primary_wallet: new_wallet_id,
                                primary_wallet,
                                network: metadata.network,
                                invite_code: invite_code.clone(),
                                invited_code: user_request.invited_code.clone(),
                                user_status: UserStatus::Active,
                                last_login_at: t,
                                last_reward_at: 0,
                                created_at: t,
                                updated_at: t,
                            };

                            let create_res = login_or_create_user::serve(new_user);

                            //TODO  增加邀请奖励
                            match create_res {
                                Ok(user) => {
                                    if let Some(code) = user_request.invited_code {
                                        let invite_user_res =
                                            repositories::user::get_user_by_invite_code(code);
                                        if let Some(u) = invite_user_res {
                                            let reward = Reward {
                                                user_id: user.user_id,
                                                reward_type: RewardType::Invite,
                                                reward_status: RewardStaus::Init,
                                                invite_user: Some(u),
                                                twiter: None,
                                                create_time: t,
                                            };
                                            let _ = add_reward(reward).await;
                                        }
                                    }
                                    Ok(user)
                                }
                                Err(e) => Err(e),
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
            }
        }
        Err(e) => Err(e),
    }
}

/// Returns the first wallet if wallet already created, or
/// Create a new wallet and return
// Commment for ii canister wallet 2024-08-13
async fn get_or_create_my_wallet(name: Option<String>) -> Result<Principal, Error> {
    let my_wallets = my_wallets();
    if my_wallets.is_empty() {
        //  create_wallet()
        // create_wallet_canister("wallet1".to_string()).await
        check_dbank_and_create_wallet(name.unwrap_or("dbank-wallet-1".to_string())).await
    } else {
        Ok(my_wallets[0].wallet_canister)
    }
}

/// Create a new walllet in dbank
async fn check_dbank_and_create_wallet(name: String) -> Result<CanisterId, Error> {
    let os = ic_cdk::id();
    let owner = ic_cdk::caller();
    let created_at = ic_cdk::api::time();

    let metadata = get_metadata();
    let steward_canister = metadata.steward_canister;
    let network = metadata.network;

    // Check wallet count of the caller
    check_wallet_count(owner)?;

    // Check the dbank canister is exist or not, if not, create it
    // Check the dbank wallet count is exceed MAX_LIMIT, if yes, create new dbank canister
    let dbank_canister = current_dbank_canister();

    let seq_in_os = wallet_counter_increment_one::serve()? as u64;
    let req = CreateDBankWalletRequest {
        seq_in_os,
        name: name.clone(),
        wallet_owner: owner,
    };

    let dbank_canister = match dbank_canister {
        Some(info) => {
            if info.is_full() {
                create_dbank_canister::serve(
                    info.dbank_id + 1,
                    seq_in_os,
                    MAX_WALLET_LIMIT,
                    name.clone(),
                    os,
                    owner,
                    metadata,
                    DBANK_WASM.to_owned(),
                )
                .await
                .map_err(|msg| Error::CreateCanisterFailed { msg })?
            } else {
                info.dbank_canister
            }
        }
        None => create_dbank_canister::serve(
            START_ID_IN_DBANK_ID,
            seq_in_os,
            MAX_WALLET_LIMIT,
            name.clone(),
            os,
            owner,
            metadata,
            DBANK_WASM.to_owned(),
        )
        .await
        .map_err(|msg| Error::CreateCanisterFailed { msg })?,
    };

    let bitcoin_address = create_dbank_wallet(dbank_canister, req).await?;
    append_wallet_action::serve(owner, Action::Create, created_at)?;

    // let wallet_address = fetch_wallet_address(wallet_canister).await?;

    let wallet_info = WalletInfo {
        name,
        owner,
        wallet_canister: dbank_canister,
        bitcoin_address,
        network,
        steward_canister,
        created_at,
        mode: Some(crate::domain::WalletMode::Shared),
    };

    // TODO: save dbank wallet
    registry_wallet::serve(wallet_info)?;

    // Ok(wallet_canister)
    Ok(dbank_canister)
}

/// Check dbank exist or not
/// Check wallet in dbank is exceed MAX_LIMIT or not
fn current_dbank_canister() -> Option<DBankInfo> {
    repositories::dbank_info::current_dbank_info()
}

async fn create_dbank_wallet(
    dbank_canister: CanisterId,
    req: CreateDBankWalletRequest,
) -> Result<String, Error> {
    let resp: Result<(String,), _> =
        ic_cdk::call(dbank_canister, "create_p2wpkh_wallet", (req,)).await;

    resp.map(|(a,)| a)
        .map_err(|(_, msg)| Error::WalletNotFound(msg))
}

#[ic_cdk::update]
async fn update_login_reward(user_id: Principal) -> Result<bool, Error> {
    // ic_cdk::print("update login -1");
    let user = get_user_info(user_id);
    // ic_cdk::print("update login  0");
    match user {
        Some(u) => {
            // ic_cdk::print("update login  1");
            let t = ic_time();
            let last_reward_time = u.last_reward_at;
            //86400 * 1000000000
            if t - last_reward_time > constants::REWARD_PERIOD {
                // ic_cdk::print("update login  2");
                let new_user = UserInfo {
                    //TODO  test
                    last_reward_at: t,
                    ..u
                };
                let update_res = login_or_create_user::serve(new_user);
                match update_res {
                    Ok(uu) => {
                        // ic_cdk::print("update login  3");
                        //TODO  增加登录奖励
                        let reward = Reward {
                            user_id: uu.user_id,
                            reward_type: RewardType::Login,
                            reward_status: RewardStaus::Confirmed,
                            invite_user: None,
                            twiter: None,
                            create_time: t,
                        };
                        let _ = add_reward(reward).await;
                        // ic_cdk::print("update login  4");
                        Ok(true)
                    }
                    Err(e) => Err(e),
                }
            } else {
                Ok(false)
            }
        }
        None => Err(Error::UnvalidParam {
            message: "user id not unvalid ".to_string(),
        }),
    }
}

#[ic_cdk::update]
async fn update_invited_code(user_id: Principal, code: String) -> Result<bool, Error> {
    let invite_user = repositories::user::get_user_by_invite_code(code.clone());
    if let Some(u) = invite_user {
        let res = update_invited_code::serve(user_id, code.clone());
        if res.is_ok() {
            let t = ic_time();
            let reward = Reward {
                user_id,
                reward_type: RewardType::Invite,
                reward_status: RewardStaus::Init,
                invite_user: Some(u),
                twiter: None,
                create_time: t,
            };
            add_reward(reward).await;
        }
        res
    } else {
        Err(Error::InviteCodeError { code })
    }
}

async fn add_reward(r: Reward) -> bool {
    log!("add reward 0");
    let point = repositories::canister_module::get_canister_module("point".to_string());
    log!("add reward 1");
    if let Some(p) = point {
        log!("add reward 2");
        let reward_res: Result<(Result<bool, String>,), _> =
            ic_cdk::call(p.canister_id, "add_reward", ((r),)).await;
        log!("add reward 3");
        match reward_res {
            Ok(r) => match r.0 {
                Ok(rr) => {
                    log!("add reward 4");
                    rr
                }
                Err(e) => {
                    log!(format!("error:add reward:{}", e));
                    false
                }
            },
            Err(e) => {
                log!(format!("error:add reward:{}", e.1));
                false
            }
        }
    } else {
        ic_cdk::print("error:add reward:point module unregister");
        false
    }
}

/***  TODO  delete when online */
#[ic_cdk::update]
fn remove(user_id: Principal) {
    let caller = ic_cdk::caller();
    if is_controller(&caller) {
        repositories::user::delete_user_info(user_id);
    }
}

#[ic_cdk::update]
fn remove_invited(user_id: Principal) {
    let caller = ic_cdk::caller();
    if is_controller(&caller) {
        let user = get_user_info(user_id).unwrap();
        let new_user = UserInfo {
            invited_code: None,
            ..user
        };
        let _ = repositories::user::update_user_info(new_user);
    }
}

#[ic_cdk::update]
fn bind_wallet(bind_request: BindRequest) -> Result<bool, Error> {
    bind_wallet::serve(bind_request)
}

#[ic_cdk::update]
fn unbind_wallet(bind_request: BindRequest) -> Result<bool, Error> {
    unbind_wallet::serve(bind_request)
}

#[ic_cdk::update]
fn add_image_link(image_link: String) {
    repositories::user::add_image_link(image_link)
}

#[ic_cdk::update]
fn update_user_profile(usrer_profile_req: UserProfileRequest) -> Result<bool, Error> {
    update_user_profile::serve(usrer_profile_req)
}
/// --------------------- Queries interface of this canister -------------------
///
/// Returns wallet counter, it will always increment by one
///
#[ic_cdk::query]
fn check_wallet_bind_status(user_id: Principal, wallet_address: String) -> bool {
    check_wallet_bind_user::serve(user_id, wallet_address)
}
#[ic_cdk::query]
fn wallet_counter() -> u128 {
    repositories::wallet_counter::get_counter()
}

/// Returns the count of wallet created by os canister
#[ic_cdk::query]
fn count_wallet() -> u64 {
    count_wallet::serve()
}

/// Returns the wallet info list of the caller
#[ic_cdk::query]
fn my_wallets() -> Vec<WalletInfo> {
    let sender = ic_cdk::caller();
    my_wallet::serve(sender)
}

/// Returns the list of wallet types
#[ic_cdk::query]
fn list_wallet_type() -> Vec<String> {
    list_wallet_types::serve()
}

/// Returns the list of wallets created by os canister
#[ic_cdk::query]
fn list_wallet() -> Vec<WalletInfo> {
    list_wallet::serve()
}

/// Returns staking pool counter
#[ic_cdk::query]
fn count_staking_pool() -> u128 {
    staking_pool_counter::get_counter()
}

/// Returns the list of staking pools
#[ic_cdk::query]
fn list_staking_pool() -> Vec<StakingPoolInfo> {
    list_staking_pool::serve()
}

/// Returns dbank list
#[ic_cdk::query]
fn list_dbank() -> Vec<DBankInfo> {
    list_dbank::serve()
}

/// Returns current dbank
#[ic_cdk::query]
fn current_dbank() -> Option<DBankInfo> {
    current_dbank::serve()
}

/// Returns total tvl of all staking pools
#[ic_cdk::query]
fn tvl() -> Satoshi {
    tvl::serve()
}

/// Returns total tvl for given staking pool
#[ic_cdk::query]
fn tvl_of(staking_canister: CanisterId) -> Satoshi {
    tvl_of::serve(staking_canister)
}

/// Returns total users  
#[ic_cdk::query]
fn total_user_count() -> u128 {
    get_user_count::serve()
}

/// Gets the staking record for given txid
#[ic_cdk::query]
fn get_staking_record(txid: String) -> Option<StakingRecord> {
    get_staking_record::serve(&txid)
}

/// Returns the list of staking records
#[ic_cdk::query]
fn list_staking_record() -> Vec<StakingRecord> {
    list_staking_record::serve()
}

/// Returns the create wallet action for given index
#[ic_cdk::query]
fn get_wallet_action(idx: u64) -> Option<WalletAction> {
    get_wallet_action::serve(idx)
}

/// Returns metadata of os canister
#[ic_cdk::query]
fn metadata() -> Metadata {
    get_metadata()
}

/// Returns the timestamp of this canister
#[ic_cdk::query]
fn timestamp() -> u64 {
    ic_cdk::api::time()
}

/// Returns the canister id  of  canister module
#[ic_cdk::query]
async fn canister_id(canister_name: String) -> Option<CanisterModuleInfo> {
    get_canister_module::serve(canister_name).await
}

#[ic_cdk::query]
fn get_user_info(user_id: Principal) -> Option<UserInfo> {
    get_user_info::serve(user_id)
}
#[ic_cdk::query]
fn get_user_by_code(code: String) -> Option<Principal> {
    repositories::user::get_user_by_invite_code(code)
}
#[ic_cdk::query]
fn get_bind_wallets_by_user(user_id: Principal) -> Option<Vec<WalletBindInfo>> {
    get_bind_wallet::serve(user_id)
}

#[ic_cdk::update]
async fn get_user_id_by_wallet(address: String) -> Result<Principal, Error> {
    let siwb_canister =
        repositories::canister_module::get_canister_module(constants::SIWB_NAME.to_string());
    if let Some(sw_module) = siwb_canister {
        let user_id_ic = wallet::utils::get_siwb_principal(sw_module.canister_id, address).await;
        Ok(user_id_ic)
    } else {
        Err(Error::UnvalidParam {
            message: "siwb module need register".to_string(),
        })
    }
}

#[ic_cdk::query]
fn get_image_link() -> Vec<String> {
    repositories::user::get_image_link()
}

#[init]
fn init(args: InitArgument) {
    STATE.with(|s| {
        let state = &mut s.borrow_mut();
        state
            .metadata
            .set(Metadata {
                network: args.network,
                steward_canister: args.steward_canister,
                wallet_cycles: args.wallet_cycles.unwrap_or(DEFAULT_CYCLES_PER_CANISTER),
                dbank_cycles: args.dbank_cycles.or(Some(DEFAULT_CYCLES_PER_DBANK)),
            })
            .expect("Failed to init metadata of os canister");
    });
}

export_candid!();

async fn fetch_wallet_address(staking_pool_canister: CanisterId) -> Result<String, Error> {
    let resp: Result<(String,), _> =
        ic_cdk::call(staking_pool_canister, "p2wsh_multisig22_address", ((),))
            .await
            .map_err(|msg| Error::GetStakingPoolAddressFailed {
                msg: format!("{msg:?}"),
            });

    resp.map(|(address,)| address)
}

fn get_metadata() -> Metadata {
    repositories::metadata::get_metadata()
}

/// Validate the given ownerr if it is owner of canister, return `Metadata` if true,
/// otherwise return `UnAuthorized`
fn validate_owner(owner: Principal) -> Result<Metadata, StakingError> {
    check_normal_principal(owner)?;

    if is_controller(&owner) {
        Ok(repositories::metadata::get_metadata())
    } else {
        Err(StakingError::UnAuthorized(owner.to_string()))
    }
}

fn check_wallet_count(owner: Principal) -> Result<(), Error> {
    if repositories::wallet_info::count_wallet_by_owner(owner) >= MAX_WALLET_PER_USER {
        return Err(Error::UnAuthorized(format!(
            "Too many wallets created: {:?}, by {}",
            MAX_WALLET_PER_USER, owner
        )));
    }
    Ok(())
}

async fn check_user_params(user_request: UserRequest) -> Result<bool, Error> {
    //TODO
    let _res = if let Some(code) = user_request.invited_code {
        let invite_user = repositories::user::get_user_by_invite_code(code.clone());
        match invite_user {
            Some(_) => Ok(true),
            None => Err(Error::InviteCodeNotExists { code }),
        }
    } else {
        Ok(true)
    };

    match user_request.user_type {
        UserType::Wallet => match user_request.wallet_address {
            Some(w) => {
                let siwb_canister = repositories::canister_module::get_canister_module(
                    constants::SIWB_NAME.to_string(),
                );
                if let Some(sw_module) = siwb_canister {
                    let user_id_ic =
                        wallet::utils::get_siwb_principal(sw_module.canister_id, w).await;
                    if wallet::utils::is_anonymous(user_id_ic) {
                        Err(Error::UnvalidParam {
                            message: "wallet address not register".to_string(),
                        })
                    } else if user_id_ic == user_request.user_id {
                        Ok(true)
                    } else {
                        Err(Error::UnvalidParam {
                            message: "user id  is wrong ".to_string(),
                        })
                    }
                } else {
                    Err(Error::UnvalidParam {
                        message: "siwb module need register".to_string(),
                    })
                }
            }
            None => Err(Error::UnvalidParam {
                message: "required wallet address".to_string(),
            }),
        },
        UserType::II => {
            let caller = ic_caller();
            if user_request.user_id == caller {
                Ok(true)
            } else {
                Err(Error::UnvalidParam {
                    message: "user id  is unvalid ".to_string(),
                })
            }
        }
    }
}
