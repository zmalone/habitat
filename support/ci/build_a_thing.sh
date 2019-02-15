#!/bin/bash

set -xeou pipefail

component=${1?component argument required}

# Since we are only verifying we don't have build failures, make everything
# temp!
export HAB_ORIGIN
HAB_ORIGIN=throwaway
# let's make a selfcontained tempdir for this job
export JOB_TEMP_ROOT
JOB_TEMP_ROOT=$(mktemp -d /tmp/job-root-XXXXXX)
export HAB_CACHE_KEY_PATH
HAB_CACHE_KEY_PATH="$JOB_TEMP_ROOT/keys"
export HAB_STUDIOS_HOME
HAB_STUDIOS_HOME="$JOB_TEMP_ROOT/studios"

hab origin key generate
hab pkg build -D components/$component