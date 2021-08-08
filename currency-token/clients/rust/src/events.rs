use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_event_hub_macros::Event;

use crate::types::{Controllers, Payload, TokenInfo};

#[derive(Event, CandidType, Deserialize)]
pub struct TokenMoveEvent {
    #[topic]
    pub from: Option<Principal>,
    #[topic]
    pub to: Option<Principal>,
    pub qty: u64,
    pub event_payload: Payload,
}

#[derive(Debug, CandidType, Deserialize)]
pub enum ControllerType {
    Mint,
    Info,
    EventListeners,
}

#[derive(Event, CandidType, Deserialize)]
pub struct ControllersUpdateEvent {
    #[topic]
    pub kind: ControllerType,
    pub new_controllers: Controllers,
}

#[derive(Event, CandidType, Deserialize)]
pub struct InfoUpdateEvent {
    pub new_info: TokenInfo,
}
