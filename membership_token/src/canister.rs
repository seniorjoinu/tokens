use ic_cdk::caller;
use ic_cdk::export::candid::export_service;
use ic_cdk_macros::{init, query, update};
use ic_event_hub::{
    implement_add_event_listeners, implement_become_event_listener, implement_event_emitter,
    implement_get_event_listeners, implement_remove_event_listeners,
};

use crate::common::api::{
    AcceptDeclineMembershipResponse, ControllerType, ControllerUpdateEvent, GetControllersResponse,
    GetTotalMembersResponse, IsMemberRequest, IsMemberResponse, IssueRevokeMembershipsRequest,
    IssueRevokeMembershipsResponse, UpdateControllerRequest, UpdateControllerResponse,
};
use crate::common::guards::{event_listeners_guard, issue_guard, revoke_guard};
use crate::common::membership_token::MembershipToken;
use crate::common::types::{Account, Controllers};
use crate::common::utils::log;

mod common;

// -------------------- MAIN LOGIC ------------------------

#[init]
fn init() {
    log("membership_token.init()");

    let controllers = Controllers::single(Account::Some(caller()));
    let token = MembershipToken::new(controllers);

    unsafe {
        STATE = Some(token);
    }
}

#[update(guard = "issue_guard")]
fn issue_memberships(request: IssueRevokeMembershipsRequest) -> IssueRevokeMembershipsResponse {
    log("membership_token.issue_memberships()");

    let state = get_state();

    request
        .into_iter()
        .map(|to| state.issue_membership(to).map(emit))
        .collect()
}

#[update(guard = "revoke_guard")]
fn revoke_memberships(request: IssueRevokeMembershipsRequest) -> IssueRevokeMembershipsResponse {
    log("membership_token.revoke_memberships()");

    let state = get_state();

    request
        .into_iter()
        .map(|from| {
            state.revoke_membership(from).map(|(ev1, ev2)| {
                emit(ev1);
                emit(ev2);
            })
        })
        .collect()
}

#[update]
fn accept_membership() -> AcceptDeclineMembershipResponse {
    log("membership_token.accept_membership()");

    get_state().accept_membership(caller()).map(|(ev1, ev2)| {
        emit(ev1);
        emit(ev2);
    })
}

#[update]
fn decline_membership() -> AcceptDeclineMembershipResponse {
    log("membership_token.decline_membership()");

    get_state().decline_membership(caller()).map(emit)
}

#[query]
fn is_member(request: IsMemberRequest) -> IsMemberResponse {
    log("membership_token.is_member()");

    get_state().is_member(&request)
}

#[query]
fn is_pending_member(request: IsMemberRequest) -> IsMemberResponse {
    log("membership_token.is_pending_member()");

    get_state().is_pending_member(&request)
}

#[query]
fn get_total_members() -> GetTotalMembersResponse {
    log("membership_token.total_members()");

    get_state().get_total_members() as u64
}

// ------------- GRANULAR CONTROL -----------------

#[update(guard = "issue_guard")]
fn update_issue_controller(request: UpdateControllerRequest) -> UpdateControllerResponse {
    log("membership_token.update_issue_controller()");

    let res = get_state().update_issue_controller(request);

    emit(ControllerUpdateEvent {
        kind: ControllerType::Issue,
        old_controller: res,
        new_controller: request,
    });

    res
}

#[update(guard = "revoke_guard")]
fn update_revoke_controller(request: UpdateControllerRequest) -> UpdateControllerResponse {
    log("membership_token.update_revoke_controller()");

    let res = get_state().update_revoke_controller(request);

    emit(ControllerUpdateEvent {
        kind: ControllerType::Revoke,
        old_controller: res,
        new_controller: request,
    });

    res
}

#[update(guard = "event_listeners_guard")]
fn update_event_listeners_controller(request: UpdateControllerRequest) -> UpdateControllerResponse {
    log("membership_token.update_event_listeners_controller()");

    let res = get_state().update_event_listeners_controller(request);

    emit(ControllerUpdateEvent {
        kind: ControllerType::EventListeners,
        old_controller: res,
        new_controller: request,
    });

    res
}

#[query]
fn get_controllers() -> GetControllersResponse {
    log("membership_token.get_controllers()");

    get_state().controllers.clone()
}

// ------------------ EVENT HUB --------------------

implement_event_emitter!();
implement_add_event_listeners!(guard = "event_listeners_guard");
implement_remove_event_listeners!(guard = "event_listeners_guard");
implement_become_event_listener!();
implement_get_event_listeners!();

// ------------------ STATE ----------------------

export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

static mut STATE: Option<MembershipToken> = None;

pub fn get_state() -> &'static mut MembershipToken {
    unsafe { STATE.as_mut().unwrap() }
}