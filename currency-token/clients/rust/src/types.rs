use ic_cdk::export::candid::{CandidType, Deserialize, Principal};

pub type Controllers = Vec<Principal>;
pub type Payload = Option<Vec<u8>>;

#[derive(Clone, CandidType, Deserialize)]
pub struct ControllerList {
    pub mint_controllers: Controllers,
    pub info_controllers: Controllers,
}

impl ControllerList {
    pub fn single(controller: Option<Principal>) -> ControllerList {
        let controllers = if controller.is_some() {
            vec![controller.unwrap()]
        } else {
            Vec::new()
        };

        ControllerList {
            mint_controllers: controllers.clone(),
            info_controllers: controllers,
        }
    }
}

#[derive(CandidType, Deserialize)]
pub struct CurrencyTokenTransferEntry {
    pub to: Principal,
    pub qty: u64,
    pub payload: Payload,
}

#[derive(Clone, CandidType, Deserialize)]
pub struct CurrencyTokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(CandidType, Deserialize)]
pub enum Error {
    InsufficientBalance,
    ZeroQuantity,
    AccessDenied,
    ForbiddenOperation,
}

#[derive(CandidType, Deserialize)]
pub struct CurrencyTokenInitRequest {
    pub info: CurrencyTokenInfo,
}

#[derive(CandidType, Deserialize)]
pub struct GetBalanceOfRequest {
    pub account_owner: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct GetBalanceOfResponse {
    pub balance: u64,
}

#[derive(CandidType, Deserialize)]
pub struct GetTotalSupplyResponse {
    pub total_supply: u64,
}

#[derive(CandidType, Deserialize)]
pub struct GetInfoResponse {
    pub info: CurrencyTokenInfo,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateInfoRequest {
    pub new_info: CurrencyTokenInfo,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateInfoResponse {
    pub old_info: CurrencyTokenInfo,
}

#[derive(CandidType, Deserialize)]
pub struct GetControllersResponse {
    pub controllers: ControllerList,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateControllersRequest {
    pub new_controllers: Controllers,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateControllersResponse {
    pub old_controllers: Controllers,
}

#[derive(CandidType, Deserialize)]
pub struct TransferRequest {
    pub entries: Vec<CurrencyTokenTransferEntry>,
}

#[derive(CandidType, Deserialize)]
pub struct TransferResponse {
    pub results: Vec<Result<(), Error>>,
}

#[derive(CandidType, Deserialize)]
pub struct BurnRequest {
    pub quantity: u64,
    pub payload: Payload,
}

pub type BurnResponse = Result<(), Error>;
