#!/usr/bin/env bash

casper-client get-deploy \
  --node-address ${NODE_ADDR} \
  $1
