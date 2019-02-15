#!/bin/bash

set -xeou pipefail

component=${1?component argument required}

export HAB_ORIGIN
HAB_ORIGIN=throwaway
# is this going to cause a problem if we have multiple agents on the same machine?
hab origin key generate
hab pkg build -D components/$component