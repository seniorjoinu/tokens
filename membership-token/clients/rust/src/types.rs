use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use std::fmt::{Display, Formatter};

#[derive(CandidType, Deserialize)]
pub enum Error {
    AlreadyIsAMember,
    IsNotAMember,
    AccessDenied,
    ForbiddenOperation,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Error::AlreadyIsAMember => "AlreadyIsAMember",
            Error::IsNotAMember => "IsNotAMember",
            Error::AccessDenied => "AccessDenied",
            Error::ForbiddenOperation => "ForbiddenOperation",
        };

        f.write_str(str)
    }
}

pub type Controllers = Vec<Principal>;

#[derive(Clone, CandidType, Deserialize)]
pub struct ControllerList {
    pub issue_controllers: Controllers,
    pub revoke_controllers: Controllers,
    pub event_listeners_controllers: Controllers,
}

impl ControllerList {
    pub fn single(controller: Option<Principal>) -> ControllerList {
        let controllers = if controller.is_some() {
            vec![controller.unwrap()]
        } else {
            Vec::new()
        };

        ControllerList {
            issue_controllers: controllers.clone(),
            revoke_controllers: controllers.clone(),
            event_listeners_controllers: controllers,
        }
    }
}

#[derive(CandidType, Deserialize)]
pub struct IsMemberRequest {
    pub prin: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct IsMemberResponse {
    pub is_member: bool,
}

#[derive(CandidType, Deserialize)]
pub struct GetTotalMembersResponse {
    pub total_members: u64,
}

#[derive(CandidType, Deserialize)]
pub struct IssueRevokeMembershipsRequest {
    pub principals: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct GetControllersResponse {
    pub controllers: ControllerList,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateControllerRequest {
    pub new_controllers: Controllers,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateControllerResponse {
    pub old_controllers: Controllers,
}

#[derive(CandidType, Deserialize)]
pub struct InitRequest {
    pub default_controllers: Option<Controllers>,
}