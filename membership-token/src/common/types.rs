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
