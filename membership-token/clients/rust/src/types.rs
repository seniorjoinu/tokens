use ic_cdk::export::candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub enum Error {
    AlreadyIsAMember,
    IsNotAMember,
    AccessDenied,
    ForbiddenOperation,
}

pub type Account = Option<Principal>;

#[derive(Clone, CandidType, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct Controllers {
    pub issue_controller: Account,
    pub revoke_controller: Account,
    pub event_listeners_controller: Account,
}

impl Controllers {
    pub fn single(controller: Account) -> Controllers {
        Controllers {
            issue_controller: controller,
            revoke_controller: controller,
            event_listeners_controller: controller,
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
