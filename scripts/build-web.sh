#!/usr/bin/env bash
# Builds the browser version of ECW: target/web/ (the UI as it is plus the calculation core as
# WebAssembly, crates/ecw-wasm) and target/web/ecw.html, the same packed into ONE file that opens
# by a double click (scripts/bundle-web.py). Needs the wasm32-unknown-unknown target and a
# wasm-bindgen CLI of the same version as the `wasm-bindgen` crate in Cargo.lock:
#   rustup target add wasm32-unknown-unknown
#   cargo install wasm-bindgen-cli --version <that version> --locked
set -euo pipefail

cd "$(dirname "$0")/.."
out=target/web

locked=$(awk '/^name = "wasm-bindgen"$/ { getline; gsub(/[version = "]/, ""); print; exit }' Cargo.lock)
have=$(wasm-bindgen --version | awk '{ print $2 }')
if [ "$locked" != "$have" ]; then
    echo "wasm-bindgen CLI is $have but Cargo.lock has $locked" >&2
    exit 1
fi

cargo build -p ecw-wasm --target wasm32-unknown-unknown --release --locked

rm -rf "$out"
mkdir -p "$out"
cp -r ui/. "$out"/
wasm-bindgen --target web --no-typescript --out-dir "$out/wasm" \
    target/wasm32-unknown-unknown/release/ecw_wasm.wasm
python3 scripts/bundle-web.py "$out" "$out/ecw.html"
