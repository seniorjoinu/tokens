## IC Tokens

This repository contains source code for a number of token canister implementations as well as a client code to interact
with these tokens on-chain.

#### Current implemented tokens:

* [Membership token](./membership-token) - simple yet powerful set membership proof.
* [Currency token](./currency-token) - fully fledged token, with recurrent payments and event-based integration that
  represents a currency.

Check [e2e-test](./e2e-tests) for browser usage examples.

#### Features

* Both tokens use [IC event hub's](https://github.com/seniorjoinu/ic-event-hub) pub/sub capabilities for easier
  integration.
* Currency token also uses [IC cron's](https://github.com/seniorjoinu/ic-cron) task scheduler for recurrent payments.

#### TODOs

* Both tokens only use linear memory for storage and therefore are not upgrade-ready.
* Both tokens use untrustworthy getters (without certified variables).
* Only client code for rust is implemented by now.