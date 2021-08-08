use ic_cdk::export::candid::export_service;
use ic_cdk::{caller, trap};
use ic_cdk_macros::{init, query, update};
use ic_event_hub_macros::{
    implement_add_event_listeners, implement_event_emitter, implement_get_event_listeners,
    implement_remove_event_listeners,
};
use union_utils::log;

use antifragile_membership_token_client::events::{
    ControllerType, ControllersUpdateEvent, MembershipStatus, MembershipStatusUpdateEvent,
};
use antifragile_membership_token_client::types::{
    ControllerList, GetControllersResponse, GetTotalMembersResponse, InitRequest, IsMemberRequest,
    IsMemberResponse, IssueRevokeMembershipsRequest, UpdateControllerRequest,
    UpdateControllerResponse,
};

use crate::common::guards::{event_listeners_guard, issue_guard, revoke_guard};
use crate::common::membership_token::MembershipToken;

mod common;

// -------------------- MAIN LOGIC ------------------------

#[init]
fn init(request: InitRequest) {
    log("membership_token.init()");

    let controllers = if let Some(default_controllers) = request.default_controllers {
        ControllerList {
            issue_controllers: default_controllers.clone(),
            revoke_controllers: default_controllers.clone(),
            event_listeners_controllers: default_controllers,
        }
    } else {
        ControllerList::single(Some(caller()))
    };

    let token = MembershipToken::new(controllers);

    unsafe {
        STATE = Some(token);
    }
}

#[update(guard = "issue_guard")]
fn issue_memberships(request: IssueRevokeMembershipsRequest) {
    log("membership_token.issue_memberships()");

    let token = get_token();

    for to in request.principals.into_iter() {
        match token.issue_membership(to) {
            Ok(_) => emit(MembershipStatusUpdateEvent {
                member: to,
                new_status: MembershipStatus::Issued,
            }),
            Err(e) => {
                trap(format!("Failed to issue membership on principal {} - {}", to, e).as_str())
            }
        }
    }
}

#[update(guard = "revoke_guard")]
fn revoke_memberships(request: IssueRevokeMembershipsRequest) {
    log("membership_token.revoke_memberships()");

    let token = get_token();

    for from in request.principals.into_iter() {
        match token.revoke_membership(from) {
            Ok(_) => emit(MembershipStatusUpdateEvent {
                member: from,
                new_status: MembershipStatus::Revoked,
            }),
            Err(e) => {
                trap(format!("Failed to revoke membership on principal {} - {}", from, e).as_str())
            }
        }
    }
}

#[update]
fn accept_membership() {
    log("membership_token.accept_membership()");

    let caller = caller();

    match get_token().accept_membership(caller) {
        Ok(_) => emit(MembershipStatusUpdateEvent {
            member: caller,
            new_status: MembershipStatus::Accepted,
        }),
        Err(e) => trap(format!("Failed to accept membership for caller - {}", e).as_str()),
    }
}

#[update]
fn decline_membership() {
    log("membership_token.decline_membership()");

    let caller = caller();

    match get_token().decline_membership(caller) {
        Ok(_) => emit(MembershipStatusUpdateEvent {
            member: caller,
            new_status: MembershipStatus::Declined,
        }),
        Err(e) => trap(format!("Failed to decline membership for caller - {}", e).as_str()),
    }
}

#[query]
fn is_member(request: IsMemberRequest) -> IsMemberResponse {
    log("membership_token.is_member()");

    let is_member = get_token().is_member(&request.prin);

    IsMemberResponse { is_member }
}

#[query]
fn is_pending_member(request: IsMemberRequest) -> IsMemberResponse {
    log("membership_token.is_pending_member()");

    let is_pending_member = get_token().is_pending_member(&request.prin);

    IsMemberResponse {
        is_member: is_pending_member,
    }
}

#[query]
fn get_total_members() -> GetTotalMembersResponse {
    log("membership_token.total_members()");

    let total_members = get_token().get_total_members() as u64;

    GetTotalMembersResponse { total_members }
}

// ------------- GRANULAR CONTROL -----------------

#[update(guard = "issue_guard")]
fn update_issue_controller(request: UpdateControllerRequest) -> UpdateControllerResponse {
    log("membership_token.update_issue_controller()");

    let old_controller = get_token().update_issue_controllers(request.new_controllers.clone());

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

    let old_controller = get_token().update_revoke_controllers(request.new_controllers.clone());

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
        get_token().update_event_listeners_controllers(request.new_controllers.clone());

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

    let controllers = get_token().controllers.clone();

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

pub fn get_token() -> &'static mut MembershipToken {
    unsafe { STATE.as_mut().unwrap() }
}
