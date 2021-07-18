## Antifragile membership token

This token represents a simple set of principals which can be manipulated by any number of controllers.

#### Usage

To deploy your own copy of this token add this repository as a git submodule of your project and incorporate it into
your `dfx.json`.

To integrate your canister with already deployed token canister:

* add `antifragile-membership-token-client = "0.1.3"` (or higher version) to the `dependencies` of your `Cargo.toml`
* use `antifragile_membership_token_client::api::MembershipTokenClient` inside your integrating canister

#### Local development

From current directory type in shell `dfx deploy`