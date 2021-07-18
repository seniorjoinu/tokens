use std::collections::HashSet;

use antifragile_membership_token_client::events::{
    MembershipStatus, MembershipStatusUpdateEvent, VotingPowerUpdateEvent,
};
use antifragile_membership_token_client::types::{Account, Controllers, Error};
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize)]
pub struct MembershipToken {
    pub pending_members: HashSet<Principal>,
    pub members: HashSet<Principal>,
    pub controllers: Controllers,
}

impl MembershipToken {
    pub fn new(controllers: Controllers) -> MembershipToken {
        MembershipToken {
            pending_members: HashSet::new(),
            members: HashSet::new(),
            controllers,
        }
    }

    pub fn issue_membership(
        &mut self,
        to: Principal,
    ) -> Result<MembershipStatusUpdateEvent, Error> {
        if self.is_member(&to) || self.is_pending_member(&to) {
            return Err(Error::AlreadyIsAMember);
        }

        self.pending_members.insert(to);

        Ok(MembershipStatusUpdateEvent {
            member: to,
            new_status: MembershipStatus::Issued,
        })
    }

    pub fn accept_membership(
        &mut self,
        caller: Principal,
    ) -> Result<(MembershipStatusUpdateEvent, VotingPowerUpdateEvent), Error> {
        if !self.is_pending_member(&caller) {
            return Err(Error::IsNotAMember);
        }
        if self.is_member(&caller) {
            return Err(Error::AlreadyIsAMember);
        }

        self.pending_members.remove(&caller);
        self.members.insert(caller);

        Ok((
            MembershipStatusUpdateEvent {
                member: caller,
                new_status: MembershipStatus::Accepted,
            },
            VotingPowerUpdateEvent {
                voter: caller,
                new_voting_power: 1,
            },
        ))
    }

    pub fn decline_membership(
        &mut self,
        caller: Principal,
    ) -> Result<MembershipStatusUpdateEvent, Error> {
        if !self.is_pending_member(&caller) {
            return Err(Error::IsNotAMember);
        }
        if self.is_member(&caller) {
            return Err(Error::AlreadyIsAMember);
        }

        self.pending_members.remove(&caller);

        Ok(MembershipStatusUpdateEvent {
            member: caller,
            new_status: MembershipStatus::Declined,
        })
    }

    pub fn revoke_membership(
        &mut self,
        from: Principal,
    ) -> Result<(MembershipStatusUpdateEvent, VotingPowerUpdateEvent), Error> {
        if !self.is_member(&from) {
            return Err(Error::IsNotAMember);
        }

        self.members.remove(&from);

        Ok((
            MembershipStatusUpdateEvent {
                member: from,
                new_status: MembershipStatus::Revoked,
            },
            VotingPowerUpdateEvent {
                voter: from,
                new_voting_power: 0,
            },
        ))
    }

    pub fn get_total_members(&self) -> usize {
        self.members.len()
    }

    pub fn update_issue_controller(&mut self, new_issue_controller: Account) -> Account {
        let old_controller = self.controllers.issue_controller;
        self.controllers.issue_controller = new_issue_controller;

        old_controller
    }

    pub fn update_revoke_controller(&mut self, new_revoke_controller: Account) -> Account {
        let old_controller = self.controllers.revoke_controller;
        self.controllers.revoke_controller = new_revoke_controller;

        old_controller
    }

    pub fn update_event_listeners_controller(
        &mut self,
        new_event_listeners_controller: Account,
    ) -> Account {
        let old_controller = self.controllers.event_listeners_controller;
        self.controllers.event_listeners_controller = new_event_listeners_controller;

        old_controller
    }

    pub fn is_member(&self, holder: &Principal) -> bool {
        self.members.contains(holder)
    }
    pub fn is_pending_member(&self, holder: &Principal) -> bool {
        self.pending_members.contains(holder)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use ic_cdk::export::Principal;

    use membership_token_client::events::MembershipStatus;
    use membership_token_client::types::{Account, Controllers};

    use crate::common::api::MembershipStatus;
    use crate::common::membership_token::MembershipToken;
    use crate::common::types::{Account, Controllers};

    fn random_principal() -> Principal {
        Principal::from_slice(
            &SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .to_be_bytes(),
        )
    }

    fn create_test_token() -> (MembershipToken, Account) {
        let controller = Some(random_principal());
        let token = MembershipToken::new(Controllers::single(controller));

        (token, controller)
    }

    #[test]
    fn creation_works_fine() {
        let (token, controller) = create_test_token();

        assert!(token.members.is_empty());
        assert!(token.pending_members.is_empty());

        assert_eq!(token.controllers.issue_controller, controller);
        assert_eq!(token.controllers.revoke_controller, controller);
        assert_eq!(token.controllers.event_listeners_controller, controller);
    }

    #[test]
    fn basic_issue_accept_revoke_flow_works_correctly() {
        let (mut token, controller) = create_test_token();
        let user_1 = random_principal();

        let event_1 = token
            .issue_membership(user_1)
            .expect("issue 1 should work just fine");

        assert!(!token.is_member(&user_1));
        assert!(token.is_pending_member(&user_1));
        assert!(token.members.is_empty());
        assert_eq!(token.pending_members.len(), 1);

        assert_eq!(event_1.member, user_1);
        assert_eq!(event_1.new_status, MembershipStatus::Issued);

        let events_2 = token
            .accept_membership(user_1)
            .expect("accept 1 should work");

        assert!(token.is_member(&user_1));
        assert!(!token.is_pending_member(&user_1));
        assert!(token.pending_members.is_empty());
        assert_eq!(token.members.len(), 1);

        let (event_2_m, event_2_v) = events_2;

        assert_eq!(event_2_m.member, user_1);
        assert_eq!(event_2_m.new_status, MembershipStatus::Accepted);

        assert_eq!(event_2_v.voter, user_1);
        assert_eq!(event_2_v.new_voting_power, 1);

        let events_3 = token
            .revoke_membership(user_1)
            .expect("revoke 1 should work");

        assert!(!token.is_member(&user_1));
        assert!(!token.is_pending_member(&user_1));
        assert!(token.members.is_empty());
        assert!(token.pending_members.is_empty());

        let (event_3_m, event_3_v) = events_3;

        assert_eq!(event_3_m.member, user_1);
        assert_eq!(event_3_m.new_status, MembershipStatus::Revoked);

        assert_eq!(event_3_v.voter, user_1);
        assert_eq!(event_3_v.new_voting_power, 0);
    }

    #[test]
    fn should_be_unable_to_revoke_not_a_member() {
        let (mut token, controller) = create_test_token();
        let user_1 = random_principal();
        let user_2 = random_principal();

        token.issue_membership(user_1);
        token.accept_membership(user_1);

        token
            .revoke_membership(user_2)
            .expect_err("revoke 1 should fail");

        token.issue_membership(user_2);

        token
            .revoke_membership(user_2)
            .expect_err("revoke 2 should fail");

        token.accept_membership(user_2);

        token
            .revoke_membership(user_2)
            .expect("revoke 3 should work");
    }
}
