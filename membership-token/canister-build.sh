#!/usr/bin/env bash

cargo build --target wasm32-unknown-unknown --package membership-token && \
 ic-cdk-optimizer ./target/wasm32-unknown-unknown/debug/membership_token.wasm -o ./target/wasm32-unknown-unknown/debug/membership-token-opt.wasm
