import {expectThrowsAsync, IMembershipTokenSetup, makeMembershipTokenSetup} from "./utils";
import {Ed25519KeyIdentity} from "@dfinity/identity";
import {assert} from 'chai';
import {deleteMembershipToken, deployMembershipToken} from "./deploy";

xdescribe('membership token', () => {
    const tokenControllerId = Ed25519KeyIdentity.generate();
    const tokenHolder1Id = Ed25519KeyIdentity.generate();
    const tokenHolder2Id = Ed25519KeyIdentity.generate();

    let tokenController: IMembershipTokenSetup;
    let tokenHolder1: IMembershipTokenSetup;
    let tokenHolder2: IMembershipTokenSetup;

    before(async () => {
        await deployMembershipToken(tokenControllerId.getPrincipal());

        tokenController = await makeMembershipTokenSetup(tokenControllerId);
        tokenHolder1 = await makeMembershipTokenSetup(tokenHolder1Id);
        tokenHolder2 = await makeMembershipTokenSetup(tokenHolder2Id);

        await Promise.all([
            tokenController.agent.fetchRootKey(),
            tokenHolder1.agent.fetchRootKey(),
            tokenHolder2.agent.fetchRootKey()
        ]);
    });

    after(async () => {
        await deleteMembershipToken();
    });

    it("basic flow works fine", async () => {
        await tokenController.membershipTokenClient.issue_memberships({
            principals: [
                tokenHolder1Id.getPrincipal(),
                tokenHolder2Id.getPrincipal(),
            ]
        });

        let holder1IsPendingMember = await tokenController.membershipTokenClient.is_pending_member({prin: tokenHolder1Id.getPrincipal()});
        assert(holder1IsPendingMember.is_member, "holder 1 should be a pending member");

        let holder2IsPendingMember = await tokenController.membershipTokenClient.is_pending_member({prin: tokenHolder2Id.getPrincipal()});
        assert(holder2IsPendingMember.is_member, "holder 2 should be a pending member");

        await tokenHolder1.membershipTokenClient.accept_membership();
        await tokenHolder2.membershipTokenClient.decline_membership();

        holder1IsPendingMember = await tokenController.membershipTokenClient.is_pending_member({prin: tokenHolder1Id.getPrincipal()});
        assert(!holder1IsPendingMember.is_member, "holder 1 should not be a pending member");

        holder2IsPendingMember = await tokenController.membershipTokenClient.is_pending_member({prin: tokenHolder2Id.getPrincipal()});
        assert(!holder2IsPendingMember.is_member, "holder 2 should not be a pending member");

        let holder1IsMember = await tokenController.membershipTokenClient.is_member({prin: tokenHolder1Id.getPrincipal()});
        assert(holder1IsMember.is_member, "holder 1 should be a member");

        let holder2IsMember = await tokenController.membershipTokenClient.is_member({prin: tokenHolder2Id.getPrincipal()});
        assert(!holder2IsMember.is_member, "holder 2 should not be a member");

        await tokenController.membershipTokenClient.revoke_memberships({principals: [tokenHolder1Id.getPrincipal()]});

        holder1IsMember = await tokenController.membershipTokenClient.is_member({prin: tokenHolder1Id.getPrincipal()});
        assert(!holder1IsMember.is_member, "holder 1 should not be a member");
    });

    it("granular issue control works fine", async () => {
        await tokenController.membershipTokenClient.issue_memberships({principals: [tokenHolder1Id.getPrincipal()]});

        await expectThrowsAsync(
            tokenHolder2.membershipTokenClient.issue_memberships({principals: [tokenHolder2Id.getPrincipal()]}),
            "Should be impossible to issue membership not being the controller"
        );

        await tokenController.membershipTokenClient.update_issue_controller({new_controllers: [tokenHolder2Id.getPrincipal()]});

        await expectThrowsAsync(
            tokenController.membershipTokenClient.issue_memberships({principals: [tokenHolder2Id.getPrincipal()]}),
            "Should not be able to issue membership by the old controller"
        );

        await tokenHolder2.membershipTokenClient.issue_memberships({principals: [tokenHolder2Id.getPrincipal()]});
        await tokenHolder2.membershipTokenClient.update_issue_controller({new_controllers: []});

        await expectThrowsAsync(
            tokenController.membershipTokenClient.issue_memberships({principals: [tokenControllerId.getPrincipal()]}),
            "Should not be able to issue membership by the old controller 1"
        );

        await expectThrowsAsync(
            tokenHolder2.membershipTokenClient.issue_memberships({principals: [tokenControllerId.getPrincipal()]}),
            "Should not be able to issue membership by the old controller 2"
        );

        await tokenHolder2.membershipTokenClient.decline_membership();
        await tokenHolder1.membershipTokenClient.decline_membership();
    });
});