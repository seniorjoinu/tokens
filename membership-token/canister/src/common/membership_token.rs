use std::collections::HashSet;

use ic_cdk::export::candid::{CandidType, Deserialize, Principal};

use antifragile_membership_token_client::types::{ControllerList, Controllers, Error};

#[derive(CandidType, Deserialize)]
pub struct MembershipToken {
    pub pending_members: HashSet<Principal>,
    pub members: HashSet<Principal>,
    pub controllers: ControllerList,
}

impl MembershipToken {
    pub fn new(controllers: ControllerList) -> MembershipToken {
        MembershipToken {
            pending_members: HashSet::new(),
            members: HashSet::new(),
            controllers,
        }
    }

    pub fn issue_membership(&mut self, to: Principal) -> Result<(), Error> {
        if self.is_member(&to) || self.is_pending_member(&to) {
            return Err(Error::AlreadyIsAMember);
        }

        self.pending_members.insert(to);

        Ok(())
    }

    pub fn accept_membership(&mut self, caller: Principal) -> Result<(), Error> {
        if !self.is_pending_member(&caller) {
            return Err(Error::IsNotAMember);
        }
        if self.is_member(&caller) {
            return Err(Error::AlreadyIsAMember);
        }

        self.pending_members.remove(&caller);
        self.members.insert(caller);

        Ok(())
    }

    pub fn decline_membership(&mut self, caller: Principal) -> Result<(), Error> {
        if !self.is_pending_member(&caller) {
            return Err(Error::IsNotAMember);
        }
        if self.is_member(&caller) {
            return Err(Error::AlreadyIsAMember);
        }

        self.pending_members.remove(&caller);

        Ok(())
    }

    pub fn revoke_membership(&mut self, from: Principal) -> Result<(), Error> {
        if !self.is_member(&from) {
            return Err(Error::IsNotAMember);
        }

        self.members.remove(&from);

        Ok(())
    }

    pub fn get_total_members(&self) -> usize {
        self.members.len()
    }

    pub fn update_issue_controllers(&mut self, new_issue_controllers: Controllers) -> Controllers {
        let old_controllers = self.controllers.issue_controllers.clone();
        self.controllers.issue_controllers = new_issue_controllers;

        old_controllers
    }

    pub fn update_revoke_controllers(
        &mut self,
        new_revoke_controllers: Controllers,
    ) -> Controllers {
        let old_controllers = self.controllers.revoke_controllers.clone();
        self.controllers.revoke_controllers = new_revoke_controllers;

        old_controllers
    }

    pub fn update_event_listeners_controllers(
        &mut self,
        new_event_listeners_controllers: Controllers,
    ) -> Controllers {
        let old_controller = self.controllers.event_listeners_controllers.clone();
        self.controllers.event_listeners_controllers = new_event_listeners_controllers;

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
    use ic_cdk::export::candid::Principal;
    use union_utils::random_principal_test;

    use antifragile_membership_token_client::events::MembershipStatus;
    use antifragile_membership_token_client::types::ControllerList;

    use crate::common::membership_token::MembershipToken;

    fn create_test_token() -> (MembershipToken, Principal) {
        let controller = random_principal_test();
        let token = MembershipToken::new(ControllerList::single(Some(controller)));

        (token, controller)
    }

    #[test]
    fn creation_works_fine() {
        let (token, controller) = create_test_token();

        assert!(token.members.is_empty());
        assert!(token.pending_members.is_empty());

        assert!(token.controllers.issue_controllers.contains(&controller));
        assert!(token.controllers.revoke_controllers.contains(&controller));
        assert!(token
            .controllers
            .event_listeners_controllers
            .contains(&controller));
    }

    #[test]
    fn basic_issue_accept_revoke_flow_works_correctly() {
        let (mut token, _) = create_test_token();
        let user_1 = random_principal_test();

        let event_1 = token.issue_membership(user_1).ok().unwrap();

        assert!(!token.is_member(&user_1));
        assert!(token.is_pending_member(&user_1));
        assert!(token.members.is_empty());
        assert_eq!(token.pending_members.len(), 1);

        assert_eq!(event_1.member, user_1);
        assert_eq!(event_1.new_status as u8, MembershipStatus::Issued as u8);

        let events_2 = token.accept_membership(user_1).ok().unwrap();

        assert!(token.is_member(&user_1));
        assert!(!token.is_pending_member(&user_1));
        assert!(token.pending_members.is_empty());
        assert_eq!(token.members.len(), 1);

        let (event_2_m, event_2_t, event_2_v) = events_2;

        assert_eq!(event_2_m.member, user_1);
        assert_eq!(event_2_m.new_status as u8, MembershipStatus::Accepted as u8);

        assert_eq!(event_2_t.new_total_voting_power, 1);

        assert_eq!(event_2_v.voter, user_1);
        assert_eq!(event_2_v.new_voting_power, 1);

        let events_3 = token.revoke_membership(user_1).ok().unwrap();

        assert!(!token.is_member(&user_1));
        assert!(!token.is_pending_member(&user_1));
        assert!(token.members.is_empty());
        assert!(token.pending_members.is_empty());

        let (event_3_m, event_3_t, event_3_v) = events_3;

        assert_eq!(event_3_m.member, user_1);
        assert_eq!(event_3_m.new_status as u8, MembershipStatus::Revoked as u8);

        assert_eq!(event_3_t.new_total_voting_power, 0);

        assert_eq!(event_3_v.voter, user_1);
        assert_eq!(event_3_v.new_voting_power, 0);
    }

    #[test]
    fn should_be_unable_to_revoke_not_a_member() {
        let (mut token, _) = create_test_token();
        let user_1 = random_principal_test();
        let user_2 = random_principal_test();

        token.issue_membership(user_1);
        token.accept_membership(user_1);

        token.revoke_membership(user_2).err().unwrap();

        token.issue_membership(user_2);

        token.revoke_membership(user_2).err().unwrap();

        token.accept_membership(user_2);

        token.revoke_membership(user_2).ok().unwrap();
    }
}
