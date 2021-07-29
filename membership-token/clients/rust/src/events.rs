use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_event_hub_macros::Event;

use crate::types::Controllers;

#[derive(Debug, CandidType, Deserialize)]
pub enum MembershipStatus {
    Issued,
    Revoked,
    Accepted,
    Declined,
}

#[derive(Event, CandidType, Deserialize)]
pub struct MembershipStatusUpdateEvent {
    #[topic]
    pub member: Principal,
    #[topic]
    pub new_status: MembershipStatus,
}

#[derive(Debug, CandidType, Deserialize)]
pub enum ControllerType {
    Issue,
    Revoke,
    EventListeners,
}

#[derive(Event, CandidType, Deserialize)]
pub struct ControllersUpdateEvent {
    #[topic]
    pub kind: ControllerType,
    pub new_controllers: Controllers,
}
