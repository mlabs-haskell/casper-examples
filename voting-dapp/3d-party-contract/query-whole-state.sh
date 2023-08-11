#!/usr/bin/env bash

set -euo pipefail

ACCOUNT_HASH=$(casper-client account-address --public-key ${PUB_HEX})

echo $ACCOUNT_HASH

ROOT_HASH=$(casper-client get-state-root-hash --node-address ${NODE_ADDR} | jq -r ".result.state_root_hash")

echo $ROOT_HASH

casper-client query-state \
  --node-address ${NODE_ADDR} \
  --state-root-hash ${ROOT_HASH} \
  --key ${ACCOUNT_HASH} 
