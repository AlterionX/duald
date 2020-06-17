#!/bin/sh

set -eu;

SCRIPT_ROOT="${0%/*}"; # if this causes problems, use "$(dirname -- $0)"

cd $SCRIPT_ROOT;
echo "Building test_server from ${SCRIPT_ROOT}.";
wasm-pack build ../duald --out-dir ../test_server/pkg --target web;
echo "Built test_server. Launching...";
http -a 127.0.0.1;
echo "test_server shutdown."
cd -;
