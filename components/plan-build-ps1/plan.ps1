$pkg_name = "hab-plan-build-ps1"
$pkg_origin = "core"
$pkg_version = "$(Get-Content $PLAN_CONTEXT/../../VERSION)"
$pkg_maintainer = "The Habitat Maintainers <humans@habitat.sh>"
$pkg_license = @("Apache-2.0")
$pkg_source = "nosuchfile.tar.gz"
$pkg_bin_dirs = @("bin")

# No runtime or build dependencies yet
$pkg_deps = @()
$pkg_build_deps = @()

$bin = @("Habitat-Build.psm1")

function Invoke-Build {
    # Embed the release version of the program.
    (Get-Content "$PLAN_CONTEXT\bin\${bin}" -Encoding Ascii) -replace
        "@VERSION@", "$pkg_version/$pkg_release" |
        Out-File "$bin" -Encoding ascii

    (Get-Content "$PLAN_CONTEXT\bin\Habitat-Build.psd1" -Encoding Ascii) -replace
        "@VERSION@", $pkg_version |
        Out-File "Habitat-Build.psd1" -Encoding ascii        
}

function Invoke-Install {
    New-Item "$pkg_prefix\bin" -ItemType Directory -Force | Out-Null
    Copy-Item "$bin" "$pkg_prefix\bin\$bin" -Force
    Copy-Item "Habitat-Build.psd1" "$pkg_prefix\bin\Habitat-Build.psd1" -Force
}

# Turn the remaining default phases into no-ops
function Invoke-Download {}

function Invoke-Verify {}

function Invoke-Unpack {}