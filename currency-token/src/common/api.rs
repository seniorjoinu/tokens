use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_event_hub_macros::Event;

use crate::common::types::{
    Account, Controllers, CurrencyTokenInfo, CurrencyTokenTransferEntry, Error, Payload,
};

// ----------- METHODS ------------------

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

// ------------ EVENTS ------------------

#[derive(Event, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct TokenMoveEvent {
    #[topic]
    pub from: Account,
    #[topic]
    pub to: Account,
    pub qty: u64,
    pub payload: Payload,
}

#[derive(Event, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct VotingPowerUpdateEvent {
    #[topic]
    pub voter: Principal,
    pub new_voting_power: u64,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub enum ControllerType {
    Mint,
    Info,
    EventListeners,
}

#[derive(Event, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct ControllerUpdateEvent {
    #[topic]
    pub kind: ControllerType,
    pub new_controller: Account,
}

#[derive(Event, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct InfoUpdateEvent {
    pub new_info: CurrencyTokenInfo,
}
