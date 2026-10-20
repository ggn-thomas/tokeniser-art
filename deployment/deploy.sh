#!/bin/bash
set -e
cd "$(dirname "$0")/../code"
cargo build-sbf
solana program deploy --program-id BipcsQBkLotT7TQmcBwsE5EAgPgbAqcXBGytmQggpXBT ./target/deploy/tokenizeart.so