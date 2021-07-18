use ic_cdk::export::candid::{CandidType, Deserialize, Principal};

pub type Account = Option<Principal>;
pub type Payload = Option<Vec<u8>>;

#[derive(Clone, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct Controllers {
    pub mint_controller: Account,
    pub info_controller: Account,
    pub event_listeners_controller: Account,
}

impl Controllers {
    pub fn single(controller: Account) -> Controllers {
        Controllers {
            mint_controller: controller,
            info_controller: controller,
            event_listeners_controller: controller,
        }
    }
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct CurrencyTokenTransferEntry {
    pub to: Principal,
    pub qty: u64,
    pub payload: Payload,
}

#[derive(Clone, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct CurrencyTokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub enum Error {
    InsufficientBalance,
    ZeroQuantity,
    AccessDenied,
    ForbiddenOperation,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct CurrencyTokenInitRequest {
    pub info: CurrencyTokenInfo,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct GetBalanceOfRequest {
    pub account_owner: Principal,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct GetBalanceOfResponse {
    pub balance: u64,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct GetTotalSupplyResponse {
    pub total_supply: u64,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct GetInfoResponse {
    pub info: CurrencyTokenInfo,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct UpdateInfoRequest {
    pub new_info: CurrencyTokenInfo,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct UpdateInfoResponse {
    pub old_info: CurrencyTokenInfo,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct GetControllersResponse {
    pub controllers: Controllers,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct UpdateControllerRequest {
    pub new_controller: Account,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct UpdateControllerResponse {
    pub old_controller: Account,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct TransferRequest {
    pub entries: Vec<CurrencyTokenTransferEntry>,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct TransferResponse {
    pub results: Vec<Result<(), Error>>,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct BurnRequest {
    pub quantity: u64,
    pub payload: Payload,
}

pub type BurnResponse = Result<(), Error>;
