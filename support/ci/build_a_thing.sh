#!/bin/bash

set -xeou pipefail

component=${1?component argument required}
# cargo_test_command="cargo test ${features_string} -- --nocapture ${test_options:-}"

# # TODO: fix this upstream so it's already on the path and set up
# export RUSTUP_HOME=/opt/rust
# export CARGO_HOME=/home/buildkite-agent/.cargo
# export PATH=/opt/rust/bin:$PATH
# # TODO: fix this upstream, it looks like it's not saving correctly.
# sudo chown -R buildkite-agent /home/buildkite-agent
export HAB_ORIGIN
HAB_ORIGIN=throwaway
hab origin key generate
hab pkg build -D components/$component