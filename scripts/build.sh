#!/bin/bash
set -e
cd "`dirname $0`"

cargo build --target wasm32-unknown-unknown --release
mkdir -p ../out
cp ../target/wasm32-unknown-unknown/release/*.wasm ../out/.