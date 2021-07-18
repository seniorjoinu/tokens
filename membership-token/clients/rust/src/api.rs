use ic_cdk::api::call::CallResult;
use ic_cdk::call;
use ic_cdk::export::candid::Principal;

use crate::types::{
    AcceptDeclineMembershipResponse, GetControllersResponse, GetTotalMembersResponse,
    IsMemberRequest, IsMemberResponse, IssueRevokeMembershipsRequest,
    IssueRevokeMembershipsResponse, UpdateControllerRequest, UpdateControllerResponse,
};

pub struct MembershipTokenClient {
    pub canister_id: Principal,
}

impl MembershipTokenClient {
    pub fn new(canister_id: Principal) -> Self {
        MembershipTokenClient { canister_id }
    }

    pub async fn issue_memberships(
        &self,
        request: IssueRevokeMembershipsRequest,
    ) -> CallResult<(IssueRevokeMembershipsResponse,)> {
        call(self.canister_id, "issue_memberships", (request,)).await
    }

    pub async fn revoke_memberships(
        &self,
        request: IssueRevokeMembershipsRequest,
    ) -> CallResult<(IssueRevokeMembershipsResponse,)> {
        call(self.canister_id, "revoke_memberships", (request,)).await
    }

    pub async fn accept_membership(&self) -> CallResult<(AcceptDeclineMembershipResponse,)> {
        call(self.canister_id, "accept_membership", ()).await
    }

    pub async fn decline_membership(&self) -> CallResult<(AcceptDeclineMembershipResponse,)> {
        call(self.canister_id, "decline_membership", ()).await
    }

    pub async fn is_member(&self, request: IsMemberRequest) -> CallResult<(IsMemberResponse,)> {
        call(self.canister_id, "is_member", (request,)).await
    }

    pub async fn is_pending_member(
        &self,
        request: IsMemberRequest,
    ) -> CallResult<(IsMemberResponse,)> {
        call(self.canister_id, "is_pending_member", (request,)).await
    }

    pub async fn get_total_members(&self) -> CallResult<(GetTotalMembersResponse,)> {
        call(self.canister_id, "get_total_members", ()).await
    }

    pub async fn update_issue_controller(
        &self,
        request: UpdateControllerRequest,
    ) -> CallResult<(UpdateControllerResponse,)> {
        call(self.canister_id, "update_issue_controller", (request,)).await
    }

    pub async fn update_revoke_controller(
        &self,
        request: UpdateControllerRequest,
    ) -> CallResult<(UpdateControllerResponse,)> {
        call(self.canister_id, "update_revoke_controller", (request,)).await
    }

    pub async fn update_event_listeners_controller(
        &self,
        request: UpdateControllerRequest,
    ) -> CallResult<(UpdateControllerResponse,)> {
        call(
            self.canister_id,
            "update_event_listeners_controller",
            (request,),
        )
        .await
    }

    pub async fn get_controllers(&self) -> CallResult<(GetControllersResponse,)> {
        call(self.canister_id, "get_controllers", ()).await
    }
}
