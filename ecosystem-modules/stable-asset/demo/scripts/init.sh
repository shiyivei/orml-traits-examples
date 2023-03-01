#!/usr/bin/env bash

set -e

echo "*** Initializing WASM build environment"

rustup update nightly
rustup update stable
rustup target add wasm32-unknown-unknown --toolchain nightly
