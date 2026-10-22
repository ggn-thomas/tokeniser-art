#!/bin/bash
set -e
cd "$(dirname "$0")/../code"
cargo build-sbf
solana program deploy ./target/deploy/tokenizeart.so