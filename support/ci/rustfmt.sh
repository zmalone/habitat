#!/bin/bash

set -euo pipefail

export RUSTUP_HOME="/opt/rust"
export CARGO_HOME="/home/buildkite-agent/.cargo"
export PATH="/opt/rust/bin:$PATH"

cargo_fmt="cargo fmt --all -- --check"
echo "--- Running cargo fmt command: $cargo_fmt"
$cargo_fmt
