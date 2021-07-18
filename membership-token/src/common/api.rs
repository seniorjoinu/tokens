use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_event_hub_macros::Event;

use crate::common::types::{Account, Controllers, Error};

// -------------- METHODS ----------------

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct IsMemberRequest {
    pub principal: Principal,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct IsMemberResponse {
    pub is_member: bool,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct GetTotalMembersResponse {
    pub total_members: u64,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct IssueRevokeMembershipsRequest {
    pub principals: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct IssueRevokeMembershipsResponse {
    pub results: Vec<Result<(), Error>>,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct AcceptDeclineMembershipResponse {
    pub result: Result<(), Error>,
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

// ------------- EVENTS ----------------

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
