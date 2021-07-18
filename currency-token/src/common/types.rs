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
