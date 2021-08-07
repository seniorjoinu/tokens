#!/usr/bin/env bash

cargo build --target wasm32-unknown-unknown --release --package membership-token && \
 ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/membership_token.wasm -o ./target/wasm32-unknown-unknown/release/membership-token-opt.wasm
