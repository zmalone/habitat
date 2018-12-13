#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The name of the component to be built. Defaults to none
    [string]$Component
)

$ErrorActionPreference="stop"

Write-Host "--- Running cargo test on $Component"
& cd components/$Component
& cargo test --verbose