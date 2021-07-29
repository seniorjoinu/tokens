use ic_cdk::caller;
use ic_cdk::export::candid::export_service;
use ic_cdk_macros::{init, query, update};
use ic_event_hub_macros::{
    implement_add_event_listeners, implement_event_emitter, implement_get_event_listeners,
    implement_remove_event_listeners,
};
use union_utils::log;

use antifragile_membership_token_client::events::{ControllerType, ControllersUpdateEvent};
use antifragile_membership_token_client::types::{
    AcceptDeclineMembershipResponse, ControllerList, GetControllersResponse,
    GetTotalMembersResponse, IsMemberRequest, IsMemberResponse, IssueRevokeMembershipsRequest,
    IssueRevokeMembershipsResponse, UpdateControllerRequest, UpdateControllerResponse,
};

use crate::common::guards::{event_listeners_guard, issue_guard, revoke_guard};
use crate::common::membership_token::MembershipToken;

mod common;

// -------------------- MAIN LOGIC ------------------------

#[init]
fn init() {
    log("membership_token.init()");

    let controllers = ControllerList::single(Some(caller()));
    let token = MembershipToken::new(controllers);

    unsafe {
        STATE = Some(token);
    }
}

#[update(guard = "issue_guard")]
fn issue_memberships(request: IssueRevokeMembershipsRequest) -> IssueRevokeMembershipsResponse {
    log("membership_token.issue_memberships()");

    let state = get_state();

    let results: Vec<_> = request
        .principals
        .into_iter()
        .map(|to| state.issue_membership(to).map(emit))
        .collect();

    IssueRevokeMembershipsResponse { results }
}

#[update(guard = "revoke_guard")]
fn revoke_memberships(request: IssueRevokeMembershipsRequest) -> IssueRevokeMembershipsResponse {
    log("membership_token.revoke_memberships()");

    let state = get_state();

    let results: Vec<_> = request
        .principals
        .into_iter()
        .map(|from| {
            state.revoke_membership(from).map(|(ev1, ev2, ev3)| {
                emit(ev1);
                emit(ev2);
                emit(ev3);
            })
        })
        .collect();

    IssueRevokeMembershipsResponse { results }
}

#[update]
fn accept_membership() -> AcceptDeclineMembershipResponse {
    log("membership_token.accept_membership()");

    let result = get_state()
        .accept_membership(caller())
        .map(|(ev1, ev2, ev3)| {
            emit(ev1);
            emit(ev2);
            emit(ev3);
        });

    AcceptDeclineMembershipResponse { result }
}

#[update]
fn decline_membership() -> AcceptDeclineMembershipResponse {
    log("membership_token.decline_membership()");

    let result = get_state().decline_membership(caller()).map(emit);

    AcceptDeclineMembershipResponse { result }
}

#[query]
fn is_member(request: IsMemberRequest) -> IsMemberResponse {
    log("membership_token.is_member()");

    let is_member = get_state().is_member(&request.principal);

    IsMemberResponse { is_member }
}

#[query]
fn is_pending_member(request: IsMemberRequest) -> IsMemberResponse {
    log("membership_token.is_pending_member()");

    let is_pending_member = get_state().is_pending_member(&request.principal);

    IsMemberResponse {
        is_member: is_pending_member,
    }
}

#[query]
fn get_total_members() -> GetTotalMembersResponse {
    log("membership_token.total_members()");

    let total_members = get_state().get_total_members() as u64;

    GetTotalMembersResponse { total_members }
}

// ------------- GRANULAR CONTROL -----------------

#[update(guard = "issue_guard")]
fn update_issue_controller(request: UpdateControllerRequest) -> UpdateControllerResponse {
    log("membership_token.update_issue_controller()");

    let old_controller = get_state().update_issue_controllers(request.new_controllers.clone());

    emit(ControllersUpdateEvent {
        kind: ControllerType::Issue,
        new_controllers: request.new_controllers,
    });

    UpdateControllerResponse {
        old_controllers: old_controller,
    }
}

#[update(guard = "revoke_guard")]
fn update_revoke_controller(request: UpdateControllerRequest) -> UpdateControllerResponse {
    log("membership_token.update_revoke_controller()");

    let old_controller = get_state().update_revoke_controllers(request.new_controllers.clone());

    emit(ControllersUpdateEvent {
        kind: ControllerType::Revoke,
        new_controllers: request.new_controllers,
    });

    UpdateControllerResponse {
        old_controllers: old_controller,
    }
}

#[update(guard = "event_listeners_guard")]
fn update_event_listeners_controller(request: UpdateControllerRequest) -> UpdateControllerResponse {
    log("membership_token.update_event_listeners_controller()");

    let old_controller =
        get_state().update_event_listeners_controllers(request.new_controllers.clone());

    emit(ControllersUpdateEvent {
        kind: ControllerType::EventListeners,
        new_controllers: request.new_controllers,
    });

    UpdateControllerResponse {
        old_controllers: old_controller,
    }
}

#[query]
fn get_controllers() -> GetControllersResponse {
    log("membership_token.get_controllers()");

    let controllers = get_state().controllers.clone();

    GetControllersResponse { controllers }
}

// ------------------ EVENT HUB --------------------

implement_event_emitter!();
implement_add_event_listeners!(guard = "event_listeners_guard");
implement_remove_event_listeners!(guard = "event_listeners_guard");
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
