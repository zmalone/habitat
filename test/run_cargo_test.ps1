#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The name of the component to be built. Defaults to none
    [string]$Component
)

$ErrorActionPreference="stop"

Invoke-Restmethod -usebasicparsing 'https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe' -outfile 'rustup-init.exe'
& rustup-init.exe -y --default-toolchain stable-x86_64-pc-windows-msvc

Write-Host "--- Running cargo test on $Component"
& cd components/$Component
& cargo test --verbose