#!/usr/bin/env bash

# Increment counter using entry point in smart contract

set -xeuo pipefail

casper-client put-deploy \
  --node-address ${NODE_ADDR} \
  --chain-name casper-net-1 \
  --secret-key ${SECRET_KEY} \
  --payment-amount 200000000 \
  --session-name "counter" \
  --session-entry-point "counter_inc" \
  --session-arg "add_amount:i32='2'"
