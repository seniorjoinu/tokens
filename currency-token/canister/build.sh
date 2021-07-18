#!/usr/bin/env bash

cargo build --target wasm32-unknown-unknown --package currency-token && \
 ic-cdk-optimizer ./target/wasm32-unknown-unknown/debug/currency_token.wasm -o ./target/wasm32-unknown-unknown/debug/currency-token-opt.wasm
