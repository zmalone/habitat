#!/bin/bash

set -euox pipefail

# TODO: fix this upstream so it's already on the path and set up
export RUSTUP_HOME=/opt/rust
export CARGO_HOME=/home/buildkite-agent/.cargo
export PATH=/opt/rust/bin:$PATH
# TODO: fix this upstream, it looks like it's not saving correctly.
sudo chown -R buildkite-agent /home/buildkite-agent

sudo -E /opt/rust/bin/rustup component add rustfmt

cargo_fmt="cargo fmt --all -- --check"
echo "--- Running cargo fmt command: $cargo_fmt"
$cargo_fmt
