use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_event_hub_macros::Event;

use crate::common::types::{Account, Controllers, Error};

// -------------- METHODS ----------------

pub type IsMemberRequest = Principal;
pub type IsMemberResponse = bool;

pub type GetTotalMembersResponse = u64;

pub type IssueRevokeMembershipsRequest = Vec<Principal>;
pub type IssueRevokeMembershipsResponse = Vec<Result<(), Error>>;

pub type AcceptDeclineMembershipResponse = Result<(), Error>;

pub type GetControllersResponse = Controllers;

pub type UpdateControllerRequest = Account;
pub type UpdateControllerResponse = Account;

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
    pub old_controller: Account,
    pub new_controller: Account,
}
