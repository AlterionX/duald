#!/bin/sh

set -eu;

VERBOSITY=0;
SCRIPT_ROOT="${0%/*}"; # if this causes problems, use "$(dirname -- $0)"
SERVER_ARGS="";
BUILD_ARGS="";
while [ $# -ne 0 ]; do
    case "$1" in
    --server)
        while [ $# -ne 1 ] && [ "$2" != "--" ]; do
            SERVER_ARGS+="$SERVER_ARGS $2";
            shift;
        done
        shift;
        if [ $# -ne 0 ] && [ "$1" != "--" ]; then
            shift;
        fi
        ;;
    --build)
        while [ $# -ne 1 ] && [ "$2" != "--" ]; do
            BUILD_ARGS="$BUILD_ARGS $2";
            shift;
        done
        if [ $# -ne 0 ] && [ "$1" != "--" ]; then
            shift;
        fi
        shift;
        ;;
    -v)
        VERBOSITY=1;
        shift;
        ;;
    *) # unsupported flags
        echo "Error: Unsupported flag $1" >&2;
        exit 1;
        ;;
    esac
done

if [ $VERBOSITY -ne 0 ]; then
    echo "Server args:${SERVER_ARGS}";
    echo "Build args:${BUILD_ARGS}";
fi

cd $SCRIPT_ROOT;
echo "Building test_server from ${SCRIPT_ROOT}.";
if [ ${#BUILD_ARGS[@]} -eq 0 ]; then
    wasm-pack build ../duald --out-dir ../test_server/pkg --target web;
else
    echo "Custom build arguments: ${BUILD_ARGS}.";
    wasm-pack build ../duald --out-dir ../test_server/pkg --target web ${BUILD_ARGS};
fi
echo "Built test_server. Launching...";
if [ ${#SERVER_ARGS[@]} -eq 0 ]; then
    http -a 127.0.0.1;
else
    echo "Custom build arguments: ${SERVER_ARGS}.";
    http ${SERVER_ARGS};
fi
echo "test_server shutdown."
cd -;
