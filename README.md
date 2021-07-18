## Antifragile tokens

This repository contains source code for a number of token canister implementations as well as a client code to interact
with these tokens on-chain.

#### Current implemented tokens:

* [Membership token](./membership-token) - simple yet powerful set membership proof.
* [Currency token](./currency-token) - fully fledged ERC20-like token that represents a currency.

#### Features

* Both tokens use [IC event hub's](https://github.com/noil3000/ic-event-hub) pub/sub capabilities for easier
  integration.
* Both tokens support [Union's](https://github.com/noil3000/union) events standard and are DAO-ready.

#### TODOs

* Both tokens only use linear memory for storage and therefore are not upgrade-ready.
* Both tokens use untrustworthy getters (without certified variables).
* Only client code for rust is implemented by now