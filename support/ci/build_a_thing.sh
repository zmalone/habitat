#!/bin/bash

set -xeou pipefail

component=${1?component argument required}

export HAB_ORIGIN
HAB_ORIGIN=throwaway
export HAB_CACHE_KEY_PATH
HAB_CACHE_KEY_PATH=$(mktemp -d /tmp/throwaway-keys-XXXXXX)
hab origin key generate
hab pkg build -D components/$component