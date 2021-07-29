use ic_cdk::export::candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub enum Error {
    AlreadyIsAMember,
    IsNotAMember,
    AccessDenied,
    ForbiddenOperation,
}

pub type Controllers = Vec<Principal>;

#[derive(Clone, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
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
    pub controllers: ControllerList,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct UpdateControllerRequest {
    pub new_controllers: Controllers,
}

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct UpdateControllerResponse {
    pub old_controllers: Controllers,
}
