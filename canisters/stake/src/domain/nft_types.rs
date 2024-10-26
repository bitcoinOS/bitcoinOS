// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.

// use serde::Deserialize;

// #![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub enum MetadataStorageType {
    S3,
    Last,
    Fleek,
    MetaBox,
}

pub type TokenIndex = u32;
#[derive(CandidType, Deserialize)]
pub struct MetadataStorageInfo {
    pub url: String,
    pub thumb: String,
}

pub type TokenIdentifier = String;
pub type AccountIdentifier = String;
#[derive(CandidType, Deserialize)]
pub enum User {
    #[serde(rename = "principal")]
    Principal_(Principal),
    #[serde(rename = "address")]
    Address(AccountIdentifier),
}

#[derive(CandidType, Deserialize)]
pub struct AllowanceRequest {
    pub token: TokenIdentifier,
    pub owner: User,
    pub spender: Principal,
}

pub type Balance1 = candid::Nat;
#[derive(CandidType, Deserialize)]
pub enum CommonError {
    InvalidToken(TokenIdentifier),
    Other(String),
}

#[derive(CandidType, Deserialize)]
pub enum Result12 {
    #[serde(rename = "ok")]
    Ok(Balance1),
    #[serde(rename = "err")]
    Err(CommonError),
}

pub type SubAccount = serde_bytes::ByteBuf;
pub type Balance = candid::Nat;
#[derive(CandidType, Deserialize)]
pub struct ApproveRequest {
    pub token: TokenIdentifier,
    pub subaccount: Option<SubAccount>,
    pub allowance: Balance,
    pub spender: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct BalanceRequest {
    pub token: TokenIdentifier,
    pub user: User,
}

#[derive(CandidType, Deserialize)]
pub enum CommonError1 {
    InvalidToken(TokenIdentifier),
    Other(String),
}

#[derive(CandidType, Deserialize)]
pub enum BalanceResponse {
    #[serde(rename = "ok")]
    Ok(Balance),
    #[serde(rename = "err")]
    Err(CommonError1),
}

#[derive(CandidType, Deserialize)]
pub struct MintRequest {
    pub to: User,
    pub metadata: Option<serde_bytes::ByteBuf>,
}

pub type Memo = serde_bytes::ByteBuf;
#[derive(CandidType, Deserialize)]
pub struct TransferRequest {
    pub to: User,
    pub token: TokenIdentifier,
    pub notify: bool,
    pub from: User,
    pub memo: Memo,
    pub subaccount: Option<SubAccount>,
    pub amount: Balance,
}

#[derive(CandidType, Deserialize)]
pub enum TransferResponseErr {
    CannotNotify(AccountIdentifier),
    InsufficientBalance,
    InvalidToken(TokenIdentifier),
    Rejected,
    Unauthorized(AccountIdentifier),
    Other(String),
}

#[derive(CandidType, Deserialize)]
pub enum TransferResponse {
    #[serde(rename = "ok")]
    Ok(Balance),
    #[serde(rename = "err")]
    Err(TransferResponseErr),
}

pub type TokenIdentifier1 = String;
pub type AccountIdentifier1 = String;
#[derive(CandidType, Deserialize)]
pub enum Result11 {
    #[serde(rename = "ok")]
    Ok(AccountIdentifier1),
    #[serde(rename = "err")]
    Err(CommonError),
}

pub type Extension = String;
#[derive(CandidType, Deserialize)]
pub enum RarityLevel {
    TopPoint1,
    TopTwenty,
    TopFive,
    TopOne,
    Common,
}

#[derive(CandidType, Deserialize)]
pub struct Rarity {
    pub level: RarityLevel,
    pub score: f64,
    pub ranking: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum Metadata {
    #[serde(rename = "fungible")]
    Fungible {
        decimals: u8,
        metadata: Option<serde_bytes::ByteBuf>,
        name: String,
        symbol: String,
    },
    #[serde(rename = "nonfungible")]
    Nonfungible {
        metadata: Option<serde_bytes::ByteBuf>,
    },
}

#[derive(CandidType, Deserialize)]
pub struct HeaderField(pub String, pub String);

#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub body: serde_bytes::ByteBuf,
    pub headers: Vec<HeaderField>,
}

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    pub body: serde_bytes::ByteBuf,
    pub headers: Vec<HeaderField>,
    pub status_code: u16,
}

#[derive(CandidType, Deserialize)]
pub struct Property {
    pub trait_type: String,
    pub value: String,
}

#[derive(CandidType, Deserialize)]
pub enum Result1 {
    #[serde(rename = "ok")]
    Ok(Metadata),
    #[serde(rename = "err")]
    Err(CommonError),
}

#[derive(CandidType, Deserialize)]
pub enum Result2 {
    #[serde(rename = "ok")]
    Ok(Balance1),
    #[serde(rename = "err")]
    Err(CommonError),
}

#[derive(CandidType, Deserialize)]
pub enum Result3 {
    #[serde(rename = "ok")]
    Ok(Vec<TokenIndex>),
    #[serde(rename = "err")]
    Err(CommonError),
}

pub type Time = candid::Int;
#[derive(CandidType, Deserialize)]
pub struct Listing {
    pub locked: Option<Time>,
    pub seller: Principal,
    pub price: u64,
}

#[derive(CandidType, Deserialize)]
pub enum Result_ {
    #[serde(rename = "ok")]
    Ok(Vec<(TokenIndex, Option<Listing>, Option<serde_bytes::ByteBuf>)>),
    #[serde(rename = "err")]
    Err(CommonError),
}

pub struct Service(pub Principal);
impl Service {
    pub async fn accept_cycles(&self) -> Result<()> {
        ic_cdk::call(self.0, "acceptCycles", ()).await
    }
    pub async fn add_metadata_storage_type(&self, arg0: String) -> Result<()> {
        ic_cdk::call(self.0, "addMetadataStorageType", (arg0,)).await
    }
    pub async fn add_metadata_url_many(
        &self,
        arg0: Vec<(MetadataStorageType, TokenIndex, MetadataStorageInfo)>,
    ) -> Result<()> {
        ic_cdk::call(self.0, "addMetadataUrlMany", (arg0,)).await
    }
    pub async fn allowance(&self, arg0: AllowanceRequest) -> Result<(Result12,)> {
        ic_cdk::call(self.0, "allowance", (arg0,)).await
    }
    pub async fn approve(&self, arg0: ApproveRequest) -> Result<(bool,)> {
        ic_cdk::call(self.0, "approve", (arg0,)).await
    }
    pub async fn approve_all(&self, arg0: Vec<ApproveRequest>) -> Result<(Vec<TokenIndex>,)> {
        ic_cdk::call(self.0, "approveAll", (arg0,)).await
    }
    pub async fn available_cycles(&self) -> Result<(candid::Nat,)> {
        ic_cdk::call(self.0, "availableCycles", ()).await
    }
    pub async fn balance(&self, arg0: BalanceRequest) -> Result<(BalanceResponse,)> {
        ic_cdk::call(self.0, "balance", (arg0,)).await
    }
    pub async fn batch_mint_nft(&self, arg0: Vec<MintRequest>) -> Result<(Vec<TokenIndex>,)> {
        ic_cdk::call(self.0, "batchMintNFT", (arg0,)).await
    }
    pub async fn batch_transfer(
        &self,
        arg0: Vec<TransferRequest>,
    ) -> Result<(Vec<TransferResponse>,)> {
        ic_cdk::call(self.0, "batchTransfer", (arg0,)).await
    }
    pub async fn bearer(&self, arg0: TokenIdentifier1) -> Result<(Result11,)> {
        ic_cdk::call(self.0, "bearer", (arg0,)).await
    }
    pub async fn clear_properties(&self) -> Result<()> {
        ic_cdk::call(self.0, "clearProperties", ()).await
    }
    pub async fn delete_metadata_storage_type(&self, arg0: String) -> Result<()> {
        ic_cdk::call(self.0, "deleteMetadataStorageType", (arg0,)).await
    }
    pub async fn extensions(&self) -> Result<(Vec<Extension>,)> {
        ic_cdk::call(self.0, "extensions", ()).await
    }
    pub async fn get_allowances(&self) -> Result<(Vec<(TokenIndex, Principal)>,)> {
        ic_cdk::call(self.0, "getAllowances", ()).await
    }
    pub async fn get_medata_storage_type(&self) -> Result<(Vec<String>,)> {
        ic_cdk::call(self.0, "getMedataStorageType", ()).await
    }
    pub async fn get_minter(&self) -> Result<(Principal,)> {
        ic_cdk::call(self.0, "getMinter", ()).await
    }
    pub async fn get_properties(&self) -> Result<(Vec<(String, Vec<(String, candid::Nat)>)>,)> {
        ic_cdk::call(self.0, "getProperties", ()).await
    }
    pub async fn get_rarity_detail(&self) -> Result<(Vec<(TokenIndex, Rarity)>,)> {
        ic_cdk::call(self.0, "getRarityDetail", ()).await
    }
    pub async fn get_registry(&self) -> Result<(Vec<(TokenIndex, AccountIdentifier1)>,)> {
        ic_cdk::call(self.0, "getRegistry", ()).await
    }
    pub async fn get_root_bucket_id(&self) -> Result<(Option<String>,)> {
        ic_cdk::call(self.0, "getRootBucketId", ()).await
    }
    pub async fn get_score(&self) -> Result<(Vec<(TokenIndex, f64)>,)> {
        ic_cdk::call(self.0, "getScore", ()).await
    }
    pub async fn get_storage_metadata_url(
        &self,
        arg0: MetadataStorageType,
        arg1: TokenIndex,
    ) -> Result<((String, String),)> {
        ic_cdk::call(self.0, "getStorageMetadataUrl", (arg0, arg1)).await
    }
    pub async fn get_tokens(&self) -> Result<(Vec<(TokenIndex, Metadata)>,)> {
        ic_cdk::call(self.0, "getTokens", ()).await
    }
    pub async fn get_tokens_by_ids(
        &self,
        arg0: Vec<TokenIndex>,
    ) -> Result<(Vec<(TokenIndex, Metadata)>,)> {
        ic_cdk::call(self.0, "getTokensByIds", (arg0,)).await
    }
    pub async fn get_tokens_by_properties(
        &self,
        arg0: Vec<(String, Vec<String>)>,
    ) -> Result<(Vec<(TokenIndex, Metadata)>,)> {
        ic_cdk::call(self.0, "getTokensByProperties", (arg0,)).await
    }
    pub async fn http_request(&self, arg0: HttpRequest) -> Result<(HttpResponse,)> {
        ic_cdk::call(self.0, "http_request", (arg0,)).await
    }
    pub async fn init_cap(&self) -> Result<(Option<String>,)> {
        ic_cdk::call(self.0, "initCap", ()).await
    }
    pub async fn init_last_metadata(&self, arg0: TokenIndex, arg1: TokenIndex) -> Result<()> {
        ic_cdk::call(self.0, "initLastMetadata", (arg0, arg1)).await
    }
    pub async fn initproperties(&self, arg0: TokenIndex, arg1: TokenIndex) -> Result<()> {
        ic_cdk::call(self.0, "initproperties", (arg0, arg1)).await
    }
    pub async fn look_properties(&self) -> Result<(Vec<(Property, Vec<TokenIndex>)>,)> {
        ic_cdk::call(self.0, "lookProperties", ()).await
    }
    pub async fn metadata(&self, arg0: TokenIdentifier1) -> Result<(Result1,)> {
        ic_cdk::call(self.0, "metadata", (arg0,)).await
    }
    pub async fn mint_nft(&self, arg0: MintRequest) -> Result<(TokenIndex,)> {
        ic_cdk::call(self.0, "mintNFT", (arg0,)).await
    }
    pub async fn replace_metadata(
        &self,
        arg0: MetadataStorageType,
        arg1: TokenIndex,
        arg2: TokenIndex,
    ) -> Result<()> {
        ic_cdk::call(self.0, "replaceMetadata", (arg0, arg1, arg2)).await
    }
    pub async fn set_minter(&self, arg0: Principal) -> Result<()> {
        ic_cdk::call(self.0, "setMinter", (arg0,)).await
    }
    pub async fn set_rarity_detail(&self) -> Result<(Vec<(TokenIndex, Rarity)>,)> {
        ic_cdk::call(self.0, "setRarityDetail", ()).await
    }
    pub async fn set_score_of_token_id(&self) -> Result<()> {
        ic_cdk::call(self.0, "setScoreOfTokenId", ()).await
    }
    pub async fn supply(&self, arg0: TokenIdentifier1) -> Result<(Result2,)> {
        ic_cdk::call(self.0, "supply", (arg0,)).await
    }
    pub async fn tokens(&self, arg0: AccountIdentifier1) -> Result<(Result3,)> {
        ic_cdk::call(self.0, "tokens", (arg0,)).await
    }
    pub async fn tokens_ext(&self, arg0: AccountIdentifier1) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "tokens_ext", (arg0,)).await
    }
    pub async fn transfer(&self, arg0: TransferRequest) -> Result<(TransferResponse,)> {
        ic_cdk::call(self.0, "transfer", (arg0,)).await
    }
    pub async fn update_nft_name(
        &self,
        arg0: String,
        arg1: String,
        arg2: TokenIndex,
        arg3: TokenIndex,
    ) -> Result<()> {
        ic_cdk::call(self.0, "updateNFTName", (arg0, arg1, arg2, arg3)).await
    }
}
