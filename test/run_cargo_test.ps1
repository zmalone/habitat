#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The name of the component to be built. Defaults to none
    [string]$Component
)

$ErrorActionPreference="stop"

dir

pwd
# $cargo = "$env:userprofile\.cargo\bin\cargo.exe"

# Write-Host "--- Installing Visual Studio Tools"
# & hab install core/visual-cpp-build-tools-2015

# # Doing this manually for the moment as POC
# $tools_path = Invoke-Expression "hab pkg path core/visual-cpp-build-tools-2015"
# $env:VCTargetsPath="$tools_path\Program Files\MSBuild\Microsoft.Cpp\v4.0\v140"
# $env:VcInstallDir="$tools_path\Program Files\Microsoft Visual Studio 14.0\VC"
# $env:WindowsSdkDir_81="$tools_path\Windows Kits\8.1"
# $env:CLTrackerSdkPath="$tools_path\Program Files\MSBuild\14.0\bin\amd64"
# $env:CLTrackerFrameworkPath="$tools_path\Program Files\MSBuild\14.0\bin\amd64"
# $env:LinkTrackerSdkPath="$tools_path\Program Files\MSBuild\14.0\bin\amd64"
# $env:LinkTrackerFrameworkPath="$tools_path\Program Files\MSBuild\14.0\bin\amd64"
# $env:LibTrackerSdkPath="$tools_path\Program Files\MSBuild\14.0\bin\amd64"
# $env:LibTrackerFrameworkPath="$tools_path\Program Files\MSBuild\14.0\bin\amd64"
# $env:RCTrackerSdkPath="$tools_path\Program Files\MSBuild\14.0\bin\amd64"
# $env:RCTrackerFrameworkPath="$tools_path\Program Files\MSBuild\14.0\bin\amd64"
# $env:DisableRegistryUse="true"
# $env:UseEnv="true"
# $env:Path="$env:Path;C:\hab\pkgs\core\visual-cpp-build-tools-2015\14.0.25420\20181108222024\Program Files\Microsoft Visual Studio 14.0\VC\bin\amd64;C:\h b\pkgs\core\visual-cpp-build-tools-2015\14.0.25420\20181108222024\Program Files\Microsoft Visual Studio 14.0\VC\redist\x64\Microsoft.VC140.CRTaC:\hab\pkgs\core\visual-cpp-build-tools-2015\14.0.25420\20181108222024\Program Files\MSBuild\14.0\bin\amd64;C:\hab\pkgs\core\visual-cpp-build-;ools-2015\14.0.25420\20181108222024\Windows Kits\8.1\bin\x64"

# Write-Host "--- Running cargo test on $Component"
# & cd components/$Component
# & $cargo build --verbose

# if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}

$env:Path="$env:Path;$env:userprofile\.cargo\bin"

cargo --version
# Invoke-RestMethod -usebasicparsing https://aka.ms/vs/15/release/vs_buildtools.exe -outfile vs_buildtools.exe