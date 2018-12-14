#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The name of the component to be built. Defaults to none
    [string]$Component
)

$ErrorActionPreference="stop"

$current_protocols = [Net.ServicePointManager]::SecurityProtocol
try {
  [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
  Invoke-RestMethod -usebasicparsing 'https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe' -outfile 'rustup-init.exe'
}
finally {
  [Net.ServicePointManager]::SecurityProtocol = $current_protocols
}

Invoke-Expression "./rustup-init.exe -y --default-toolchain stable-x86_64-pc-windows-msvc"
$cargo = "$env:userprofile\.cargo\bin\cargo.exe"

Write-Host "--- Running cargo test on $Component"
Invoke-Expression "cd components/$Component && $cargo test --lib --verbose"

if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}