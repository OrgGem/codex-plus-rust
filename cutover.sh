#!/usr/bin/env bash
# Safe interactive & automated cutover / rollback wizard for CodexPlusPlus Pure-Rust Port (Phase 6).
# Usage:
#   ./rust-port/cutover.sh           # Interactive cutover walkthrough
#   ./rust-port/cutover.sh --check   # Non-interactive preflight & Gateway smoke-check
#   ./rust-port/cutover.sh --rollback
#   ./rust-port/cutover.sh --help

set -euo pipefail

PORT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ -f "${PORT_ROOT}/Cargo.toml" ]]; then
  WORKSPACE_ROOT="${PORT_ROOT}"
else
  WORKSPACE_ROOT="$(cd "${PORT_ROOT}/.." && pwd)"
fi
RELEASE_DIR="${PORT_ROOT}/target/release"

IS_WINDOWS=0
EXE_EXT=""
case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*)
    IS_WINDOWS=1
    EXE_EXT=".exe"
    ;;
esac

APP_BIN="${RELEASE_DIR}/codex-plus-app${EXE_EXT}"
DEVKIT_BIN="${RELEASE_DIR}/codex-plus-devkit${EXE_EXT}"
INSPECTOR_BIN="${RELEASE_DIR}/native-browser-inspector${EXE_EXT}"

print_usage() {
  echo "Usage: ./rust-port/cutover.sh [--check | --rollback | --help]"
}

if [[ $# -gt 1 ]]; then
  print_usage >&2
  exit 2
fi

case "${1:-}" in
  --help|-h)
    print_usage
    exit 0
    ;;
  --rollback)
    echo "[Rollback] Stopping codex-plus-app instances (preserving original installation)..."
    if [[ "${IS_WINDOWS}" -eq 1 ]]; then
      taskkill //F //IM codex-plus-app.exe >/dev/null 2>&1 || true
    else
      pkill -x "codex-plus-app" 2>/dev/null || true
    fi
    echo "[Rollback] Complete. Original repository remains untouched."
    exit 0
    ;;
  ""|--check)
    ;;
  *)
    echo "Unknown option: $1" >&2
    print_usage >&2
    exit 2
    ;;
esac

REPO_VERSION="$(sed -n 's/^[[:space:]]*version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "${WORKSPACE_ROOT}/Cargo.toml" | head -n 1)"
if [[ -z "${REPO_VERSION}" ]]; then
  echo "ERROR: Could not read version from ${WORKSPACE_ROOT}/Cargo.toml" >&2
  exit 1
fi

run_preflight() {
  echo "=== [1/3] Checking Pure-Rust Release Binaries (v${REPO_VERSION}) ==="
  REQUIRED_FILES=("${APP_BIN}" "${DEVKIT_BIN}" "${INSPECTOR_BIN}")
  if [[ "${IS_WINDOWS}" -eq 1 ]]; then
    REQUIRED_FILES+=("${RELEASE_DIR}/WebView2Loader.dll")
  fi
  for b in "${REQUIRED_FILES[@]}"; do
    if [[ ! -f "${b}" ]]; then
      echo "ERROR: Missing required artifact: ${b}" >&2
      exit 1
    fi
    echo "  [OK] ${b}"
  done

  echo "=== [2/3] Running codex-plus-devkit Verification ==="
  "${DEVKIT_BIN}" release-meta check "${REPO_VERSION}" --root "${WORKSPACE_ROOT}"
  "${DEVKIT_BIN}" assemble-renderer-inject --check --root "${WORKSPACE_ROOT}"
  "${DEVKIT_BIN}" relaunch-local --pure-rust --dry-run --root "${WORKSPACE_ROOT}"

  echo "=== [3/3] Running codex-plus-app --smoke-check ==="
  SMOKE_DIR="${PORT_ROOT}/target/cutover-smoke-state"
  SMOKE_JSON="${SMOKE_DIR}/smoke-report.json"
  mkdir -p "${SMOKE_DIR}"
  rm -f "${SMOKE_JSON}"
  CODEX_PLUS_STATE_DIR="${SMOKE_DIR}" \
  CODEX_PLUS_MANAGER_GUARD_PORT=63142 \
  CODEX_PLUS_LAUNCHER_GUARD_PORT=63143 \
    "${APP_BIN}" --smoke-check "${SMOKE_JSON}"
}

if [[ "${1:-}" == "--check" ]]; then
  run_preflight
  echo "[PASS] Cutover preflight succeeded."
  exit 0
fi

echo "CodexPlusPlus Pure-Rust Interactive Cutover (original kept intact for instant rollback)"
read -r -p "Step 1: Run preflight & live Gateway smoke-check? [Enter to continue, q to abort] " ans || exit 0
[[ "${ans}" == "q" ]] && exit 0
run_preflight

read -r -p "Step 2: Stage & launch Pure-Rust codex-plus-app? [Enter to continue, q to abort] " ans || exit 0
[[ "${ans}" == "q" ]] && exit 0
"${DEVKIT_BIN}" relaunch-local --pure-rust --skip-build --root "${WORKSPACE_ROOT}"

echo "[Cutover Active] Roll back anytime via: ./cutover.sh --rollback"
