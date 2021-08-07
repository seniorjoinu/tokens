#!/usr/bin/env bash

cargo build --target wasm32-unknown-unknown --release --package currency-token && \
 ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/currency_token.wasm -o ./target/wasm32-unknown-unknown/release/currency-token-opt.wasm
