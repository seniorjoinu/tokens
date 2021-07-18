use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_event_hub_macros::Event;

use crate::types::Account;

#[derive(Event, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct VotingPowerUpdateEvent {
    #[topic]
    pub voter: Principal,
    pub new_voting_power: u64,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
pub enum MembershipStatus {
    Issued,
    Revoked,
    Accepted,
    Declined,
}

#[derive(Event, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct MembershipStatusUpdateEvent {
    #[topic]
    pub member: Principal,
    #[topic]
    pub new_status: MembershipStatus,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub enum ControllerType {
    Issue,
    Revoke,
    EventListeners,
}

#[derive(Event, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct ControllerUpdateEvent {
    #[topic]
    pub kind: ControllerType,
    pub new_controller: Account,
}
