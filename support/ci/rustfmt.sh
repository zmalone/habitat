#!/bin/bash

set -euox pipefail

export RUSTUP_HOME="/opt/rust"
export CARGO_HOME="/home/buildkite-agent/.cargo"
export PATH="/opt/rust/bin:$PATH"

ls -la

which cargo

/opt/rust/bin/cargo --version

cargo_fmt="cargo fmt --all -- --check"
echo "--- Running cargo fmt command: $cargo_fmt"
$cargo_fmt
