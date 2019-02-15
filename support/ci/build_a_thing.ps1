#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The name of the component to be built. Defaults to none
    [string]$Component,
)

# Since we are only verifying we don't have build failures, make everything
# temp!
$env:HAB_ORIGIN="throwaway"
# let's make a selfcontained tempdir for this job
$job_temp_root = mkdir (Join-Path $env:TEMP ([System.IO.Path]::GetRandomFileName()))
$env:HAB_CACHE_KEY_PATH="$job_temp_root/keys"

& hab origin key generate
& hab pkg build -D components/$component