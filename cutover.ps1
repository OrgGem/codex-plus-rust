<#
.SYNOPSIS
    Safe, interactive & automated cutover / rollback wizard for CodexPlusPlus Pure-Rust Port (Phase 6).

.PARAMETER Check
    Run non-interactive preflight & live health verification only (no prompts, zero side effects).

.PARAMETER Rollback
    Stop Pure-Rust `codex-plus-app` instances cleanly and keep the original app active.
#>
param(
    [switch]$Check,
    [switch]$Rollback
)

$ErrorActionPreference = "Stop"
$PortRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Split-Path -Parent $PortRoot
$ReleaseDir = Join-Path $PortRoot "target\release"

function Get-RepoVersion {
    $cargoToml = Join-Path $RepoRoot "Cargo.toml"
    foreach ($line in (Get-Content $cargoToml)) {
        if ($line -match '^\s*version\s*=\s*"([^"]+)"') {
            return $Matches[1]
        }
    }
    throw "Could not determine version from $cargoToml"
}

$Binaries = @(
    @{ Name = "codex-plus-app.exe"; Required = $true; Desc = "Pure-Rust Dioxus Desktop Manager + Gateway" },
    @{ Name = "WebView2Loader.dll"; Required = $true; Desc = "WebView2 runtime loader (x64 GNU)" },
    @{ Name = "codex-plus-devkit.exe"; Required = $true; Desc = "Pure-Rust DevOps/Build/Release CLI" },
    @{ Name = "native-browser-inspector.exe"; Required = $true; Desc = "oxc_parser AST Browser Inspector" },
    @{ Name = "codex-wechat.exe"; Required = $false; Desc = "Pure-Rust WeChat Bridge CLI" },
    @{ Name = "claude-patcher.exe"; Required = $false; Desc = "Pure-Rust Claude Localization Patcher" },
    @{ Name = "remote-control-p0.exe"; Required = $false; Desc = "Pure-Rust Remote Control Relay CLI" }
)

if ($Rollback) {
    Write-Host "[Rollback] Stopping running codex-plus-app GUI instances..." -ForegroundColor Yellow
    Get-Process codex-plus-app -ErrorAction SilentlyContinue | Stop-Process -Force
    Write-Host "[Rollback] Done. Original repository and ~/.codex configuration remain untouched." -ForegroundColor Green
    exit 0
}

function Invoke-PreflightCheck {
    $expectedVer = Get-RepoVersion
    Write-Host "=== [1/3] Checking Pure-Rust Release Artifacts (v$expectedVer) ===" -ForegroundColor Cyan
    foreach ($item in $Binaries) {
        $full = Join-Path $ReleaseDir $item.Name
        if (Test-Path $full) {
            $info = Get-Item $full
            $hash = (Get-FileHash -Path $full -Algorithm SHA256).Hash.Substring(0, 16)
            $mb = [math]::Round($info.Length / 1MB, 2)
            Write-Host ("  [OK] {0,-30} {1,7} MB  sha256:{2}... ({3})" -f $item.Name, $mb, $hash, $item.Desc) -ForegroundColor Green
        } elseif ($item.Required) {
            throw "Required release artifact missing: $full"
        } else {
            Write-Host ("  [INFO] Optional binary not yet built in release/: {0}" -f $item.Name) -ForegroundColor DarkGray
        }
    }

    Write-Host "`n=== [2/3] Verifying codex-plus-devkit Pure-Rust Checks ===" -ForegroundColor Cyan
    $devkit = Join-Path $ReleaseDir "codex-plus-devkit.exe"
    & $devkit release-meta check $expectedVer --root $RepoRoot
    if ($LASTEXITCODE -ne 0) { throw "release-meta check $expectedVer failed" }
    & $devkit assemble-renderer-inject --check --root $RepoRoot
    if ($LASTEXITCODE -ne 0) { throw "assemble-renderer-inject --check failed" }
    & $devkit relaunch-local --pure-rust --dry-run --root $RepoRoot
    if ($LASTEXITCODE -ne 0) { throw "relaunch-local --pure-rust --dry-run failed" }

    Write-Host "`n=== [3/3] Running codex-plus-app --smoke-check (Live Gateway + Codex Probe) ===" -ForegroundColor Cyan
    $smokeState = Join-Path $PortRoot "target\cutover-smoke-state"
    $smokeOut = Join-Path $smokeState "smoke-report.json"
    New-Item -ItemType Directory -Force -Path $smokeState | Out-Null
    if (Test-Path $smokeOut) { Remove-Item -Force $smokeOut }
    $prevState = $env:CODEX_PLUS_STATE_DIR
    $prevMgr = $env:CODEX_PLUS_MANAGER_GUARD_PORT
    $prevLch = $env:CODEX_PLUS_LAUNCHER_GUARD_PORT
    try {
        $env:CODEX_PLUS_STATE_DIR = $smokeState
        $env:CODEX_PLUS_MANAGER_GUARD_PORT = "63142"
        $env:CODEX_PLUS_LAUNCHER_GUARD_PORT = "63143"
        $appExe = Join-Path $ReleaseDir "codex-plus-app.exe"
        $p = Start-Process -FilePath $appExe -ArgumentList @("--smoke-check", $smokeOut) -Wait -PassThru
        if ($p.ExitCode -ne 0) { throw "codex-plus-app --smoke-check exited with $($p.ExitCode)" }
        $jsonRaw = Get-Content -Raw $smokeOut
        $parsed = $jsonRaw | ConvertFrom-Json
        if (-not $parsed.ok -or $parsed.backendStatusHttp -ne 200) {
            throw "Smoke-check reported unhealthy Gateway state: $jsonRaw"
        }
        Write-Host ("  [OK] Smoke-check PASSED: version={0}, gatewayPort={1}, HTTP={2}, sessions={3}, presets={4}" -f `
            $parsed.version, $parsed.gatewayHelperPort, $parsed.backendStatusHttp, $parsed.localSessionsCount, $parsed.presetCount) -ForegroundColor Green
    } finally {
        $env:CODEX_PLUS_STATE_DIR = $prevState
        $env:CODEX_PLUS_MANAGER_GUARD_PORT = $prevMgr
        $env:CODEX_PLUS_LAUNCHER_GUARD_PORT = $prevLch
    }
}

if ($Check) {
    Invoke-PreflightCheck
    Write-Host "`n[PASS] Cutover preflight verification succeeded with 0 errors." -ForegroundColor Green
    exit 0
}

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host " CodexPlusPlus Pure-Rust Cutover Wizard" -ForegroundColor Cyan
Write-Host " Original codebase remains untouched as rollback fallback." -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

$ans = Read-Host "`nStep 1: Run automated binary & Gateway health preflight? [Enter = Yes, q = Quit]"
if ($null -eq $ans -or $ans -eq "q") { exit 0 }
Invoke-PreflightCheck

$ans = Read-Host "`nStep 2: Stage & launch Pure-Rust Desktop app via codex-plus-devkit? [Enter = Yes, q = Quit]"
if ($null -eq $ans -or $ans -eq "q") { exit 0 }
$devkitExe = Join-Path $ReleaseDir "codex-plus-devkit.exe"
& $devkitExe relaunch-local --pure-rust --skip-build --root $RepoRoot
if ($LASTEXITCODE -ne 0) { throw "Failed to stage/launch Pure-Rust bundle" }

Write-Host "`n[Cutover Active] Pure-Rust Codex++ Desktop is running from rust-port\target\local-bundle\" -ForegroundColor Green
Write-Host "To roll back at any time, run: powershell -ExecutionPolicy Bypass -File rust-port\cutover.ps1 -Rollback" -ForegroundColor Yellow
