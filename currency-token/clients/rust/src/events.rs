use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_event_hub_macros::Event;

use crate::types::{Account, CurrencyTokenInfo, Payload};

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
