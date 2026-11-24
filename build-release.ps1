<#
.SYNOPSIS
    Standalone Pure-Rust build, test, smoke-check, and portable release packager for CodexPlusPlus Rust.

.DESCRIPTION
    Builds the unified Cargo workspace inside this repository without requiring Node.js, npm,
    Vite, Python, or the parent monorepo. Produces a portable Windows x64 bundle in
    `release-bundle/CodexPlusPlus-Rust-v<ver>-win-x64/` along with `SHA256SUMS.txt` and `.zip`.

.PARAMETER SkipTests
    Skip running `cargo test` before building release binaries.

.PARAMETER SkipZip
    Skip compressing the portable folder into a `.zip` archive.

.PARAMETER Offline
    Pass `--offline` to Cargo commands (uses local Cargo cache only).

.PARAMETER Launch
    Launch the packaged portable `codex-plus-app.exe` after verification succeeds.
#>
param(
    [switch]$SkipTests,
    [switch]$SkipZip,
    [switch]$Offline,
    [switch]$Launch
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $RepoRoot

function Get-WorkspaceVersion {
    $cargoToml = Join-Path $RepoRoot "Cargo.toml"
    $inWsPkg = $false
    foreach ($line in (Get-Content $cargoToml)) {
        if ($line -match '^\s*\[workspace\.package\]') { $inWsPkg = $true; continue }
        if ($inWsPkg -and $line -match '^\s*\[') { break }
        if ($inWsPkg -and $line -match '^\s*version\s*=\s*"([^"]+)"') {
            return $Matches[1]
        }
    }
    throw "Could not read [workspace.package] version from $cargoToml"
}

$Version = Get-WorkspaceVersion
$CargoFlags = @()
if ($Offline) { $CargoFlags += "--offline" }

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " CodexPlusPlus Pure-Rust Standalone Release Builder (v$Version)" -ForegroundColor Cyan
Write-Host " Workspace: $RepoRoot" -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

if (-not $SkipTests) {
    Write-Host "`n[1/5] Running Pure-Rust workspace tests..." -ForegroundColor Cyan
    & cargo test @CargoFlags
    if ($LASTEXITCODE -ne 0) { throw "Workspace tests failed (exit $LASTEXITCODE)" }
    Write-Host "  [OK] All workspace tests passed." -ForegroundColor Green
} else {
    Write-Host "`n[1/5] Skipping workspace tests (-SkipTests)." -ForegroundColor Yellow
}

Write-Host "`n[2/5] Building release binaries (codex-plus-app + DevKit + satellite CLIs)..." -ForegroundColor Cyan
$BuildArgs = @(
    "build", "--release",
    "-p", "codex-plus-app",
    "-p", "codex-plus-devkit",
    "-p", "codex-wechat-rs",
    "-p", "claude-patcher-rs",
    "-p", "remote-control-p0"
) + $CargoFlags
& cargo @BuildArgs
if ($LASTEXITCODE -ne 0) { throw "Release build failed (exit $LASTEXITCODE)" }

$ReleaseDir = Join-Path $RepoRoot "target\release"
$BundleName = "CodexPlusPlus-Rust-v$Version-win-x64"
$BundleRoot = Join-Path $RepoRoot "release-bundle"
$PkgDir = Join-Path $BundleRoot $BundleName
New-Item -ItemType Directory -Force -Path $PkgDir | Out-Null

Write-Host "`n[3/5] Staging portable release bundle in $PkgDir..." -ForegroundColor Cyan
$Artifacts = @(
    @{ File = "codex-plus-app.exe"; Required = $true },
    @{ File = "WebView2Loader.dll"; Required = $true },
    @{ File = "codex-plus-devkit.exe"; Required = $true },
    @{ File = "native-browser-inspector.exe"; Required = $true },
    @{ File = "codex-wechat.exe"; Required = $true },
    @{ File = "claude-patcher.exe"; Required = $true },
    @{ File = "remote-control-p0.exe"; Required = $true }
)

foreach ($item in $Artifacts) {
    $src = Join-Path $ReleaseDir $item.File
    if (-not (Test-Path $src)) {
        if ($item.Required) { throw "Missing release artifact: $src" }
        continue
    }
    Copy-Item -Force $src (Join-Path $PkgDir $item.File)
}

$LauncherCmd = @"
@echo off
setlocal
set "APP_DIR=%~dp0"
set "CODEX_PLUS_STATE_DIR=%APP_DIR%state"
if not exist "%CODEX_PLUS_STATE_DIR%" mkdir "%CODEX_PLUS_STATE_DIR%"
start "" "%APP_DIR%codex-plus-app.exe"
"@
Set-Content -Path (Join-Path $PkgDir "Start-CodexPlusPlus-Portable.cmd") -Value $LauncherCmd -Encoding ASCII

$ReadmeTxt = @"
CodexPlusPlus Pure-Rust Standalone Edition v$Version (Windows x64)
==================================================================

Files included:
- codex-plus-app.exe               : Standalone Pure-Rust Desktop Manager + Local Gateway (Dioxus + WebView2)
- WebView2Loader.dll               : Microsoft Edge WebView2 native loader (keep beside codex-plus-app.exe)
- Start-CodexPlusPlus-Portable.cmd : Launches the app with isolated local ./state/ directory
- codex-plus-devkit.exe            : DevOps, release metadata, renderer-inject & local bundle CLI
- native-browser-inspector.exe     : Pure-Rust oxc_parser AST inspector (zero Node.js required)
- codex-wechat.exe                 : Pure-Rust WeChat iLink bridge CLI
- claude-patcher.exe               : Pure-Rust Claude Desktop localization & ASAR patcher CLI
- remote-control-p0.exe            : Pure-Rust Remote Control IPC/relay CLI
- SHA256SUMS.txt                   : SHA-256 checksums of all included files

Quick Start:
- Double-click Start-CodexPlusPlus-Portable.cmd (isolated portable state)
- Or run codex-plus-app.exe directly (uses default user CodexPlusPlus state directory)
"@
Set-Content -Path (Join-Path $PkgDir "README.txt") -Value $ReadmeTxt -Encoding UTF8

Write-Host "`n[4/5] Running live --smoke-check and generating SHA256SUMS.txt..." -ForegroundColor Cyan
$SmokeState = Join-Path $RepoRoot "target\release-smoke-state"
$SmokeOut = Join-Path $SmokeState "smoke-report.json"
New-Item -ItemType Directory -Force -Path $SmokeState | Out-Null
if (Test-Path $SmokeOut) { Remove-Item -Force $SmokeOut }

$prevState = $env:CODEX_PLUS_STATE_DIR
$prevMgr = $env:CODEX_PLUS_MANAGER_GUARD_PORT
$prevLch = $env:CODEX_PLUS_LAUNCHER_GUARD_PORT
try {
    $env:CODEX_PLUS_STATE_DIR = $SmokeState
    $env:CODEX_PLUS_MANAGER_GUARD_PORT = "63152"
    $env:CODEX_PLUS_LAUNCHER_GUARD_PORT = "63153"
    $p = Start-Process -FilePath (Join-Path $PkgDir "codex-plus-app.exe") -ArgumentList @("--smoke-check", $SmokeOut) -Wait -PassThru
    if ($p.ExitCode -ne 0) { throw "codex-plus-app --smoke-check failed with exit code $($p.ExitCode)" }
    $report = Get-Content -Raw $SmokeOut | ConvertFrom-Json
    if (-not $report.ok -or $report.backendStatusHttp -ne 200) {
        throw "Smoke-check reported unhealthy status: $(Get-Content -Raw $SmokeOut)"
    }
    Write-Host ("  [OK] Smoke-check PASSED: v{0} | gatewayPort={1} | HTTP={2} | sessions={3} | presets={4}" -f `
        $report.version, $report.gatewayHelperPort, $report.backendStatusHttp, $report.localSessionsCount, $report.presetCount) -ForegroundColor Green
} finally {
    $env:CODEX_PLUS_STATE_DIR = $prevState
    $env:CODEX_PLUS_MANAGER_GUARD_PORT = $prevMgr
    $env:CODEX_PLUS_LAUNCHER_GUARD_PORT = $prevLch
}

$HashLines = Get-ChildItem -Path $PkgDir -File |
    Where-Object { $_.Name -ne "SHA256SUMS.txt" } |
    Sort-Object Name |
    ForEach-Object {
        $h = (Get-FileHash -Path $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
        "$h  $($_.Name)"
    }
Set-Content -Path (Join-Path $PkgDir "SHA256SUMS.txt") -Value $HashLines -Encoding ASCII

if (-not $SkipZip) {
    Write-Host "`n[5/5] Creating portable ZIP archive..." -ForegroundColor Cyan
    $ZipPath = Join-Path $BundleRoot "$BundleName.zip"
    if (Test-Path $ZipPath) { Remove-Item -Force $ZipPath }
    Compress-Archive -Path (Join-Path $PkgDir "*") -DestinationPath $ZipPath -CompressionLevel Optimal
    $ZipMb = [math]::Round((Get-Item $ZipPath).Length / 1MB, 2)
    Write-Host "  [OK] Archive created: $ZipPath ($ZipMb MB)" -ForegroundColor Green
} else {
    Write-Host "`n[5/5] Skipping ZIP archive (-SkipZip)." -ForegroundColor Yellow
}

Write-Host "`n[DONE] Portable bundle ready at: $PkgDir" -ForegroundColor Green

if ($Launch) {
    Write-Host "[Launch] Starting $PkgDir\codex-plus-app.exe ..." -ForegroundColor Cyan
    $StateDir = Join-Path $PkgDir "state"
    New-Item -ItemType Directory -Force -Path $StateDir | Out-Null
    $env:CODEX_PLUS_STATE_DIR = $StateDir
    $env:CODEX_PLUS_MANAGER_GUARD_PORT = "63132"
    $env:CODEX_PLUS_LAUNCHER_GUARD_PORT = "63133"
    Start-Process -FilePath (Join-Path $PkgDir "codex-plus-app.exe") -WorkingDirectory $PkgDir
}
