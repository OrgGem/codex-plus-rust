#!/usr/bin/env bash
# Standalone Pure-Rust build, test, smoke-check, and release packager for CodexPlusPlus Rust.
# Usage:
#   ./build-release.sh [--skip-tests] [--offline]
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${REPO_ROOT}"

SKIP_TESTS=0
CARGO_FLAGS=()
for arg in "$@"; do
  case "${arg}" in
    --skip-tests) SKIP_TESTS=1 ;;
    --offline) CARGO_FLAGS+=("--offline") ;;
    --help|-h)
      echo "Usage: ./build-release.sh [--skip-tests] [--offline]"
      exit 0
      ;;
    *)
      echo "Unknown option: ${arg}" >&2
      exit 2
      ;;
  esac
done

VERSION="$(sed -n '/^\[workspace\.package\]/,/^\[/{s/^[[:space:]]*version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p;}' "${REPO_ROOT}/Cargo.toml" | head -n 1)"
if [[ -z "${VERSION}" ]]; then
  echo "ERROR: Could not read version from Cargo.toml" >&2
  exit 1
fi

OS_TAG="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH_TAG="$(uname -m)"
EXE_EXT=""
case "${OS_TAG}" in
  mingw*|msys*|cygwin*) OS_TAG="win"; EXE_EXT=".exe" ;;
esac

echo "=== CodexPlusPlus Pure-Rust Standalone Build (v${VERSION} | ${OS_TAG}-${ARCH_TAG}) ==="

if [[ "${SKIP_TESTS}" -eq 0 ]]; then
  echo "[1/4] Running workspace tests..."
  cargo test "${CARGO_FLAGS[@]}"
else
  echo "[1/4] Skipping workspace tests (--skip-tests)."
fi

echo "[2/4] Building release binaries..."
cargo build --release \
  -p codex-plus-app \
  -p codex-plus-devkit \
  -p codex-wechat-rs \
  -p claude-patcher-rs \
  -p remote-control-p0 \
  "${CARGO_FLAGS[@]}"

PKG_NAME="CodexPlusPlus-Rust-v${VERSION}-${OS_TAG}-${ARCH_TAG}"
PKG_DIR="${REPO_ROOT}/release-bundle/${PKG_NAME}"
mkdir -p "${PKG_DIR}"

echo "[3/4] Staging binaries into ${PKG_DIR}..."
for bin in codex-plus-app codex-plus-devkit native-browser-inspector codex-wechat claude-patcher remote-control-p0; do
  cp -f "${REPO_ROOT}/target/release/${bin}${EXE_EXT}" "${PKG_DIR}/"
done
if [[ -f "${REPO_ROOT}/target/release/WebView2Loader.dll" ]]; then
  cp -f "${REPO_ROOT}/target/release/WebView2Loader.dll" "${PKG_DIR}/"
fi

echo "[4/4] Running live --smoke-check and writing SHA256SUMS.txt..."
SMOKE_DIR="${REPO_ROOT}/target/release-smoke-state"
SMOKE_JSON="${SMOKE_DIR}/smoke-report.json"
mkdir -p "${SMOKE_DIR}"
rm -f "${SMOKE_JSON}"
CODEX_PLUS_STATE_DIR="${SMOKE_DIR}" \
CODEX_PLUS_MANAGER_GUARD_PORT=63152 \
CODEX_PLUS_LAUNCHER_GUARD_PORT=63153 \
  "${PKG_DIR}/codex-plus-app${EXE_EXT}" --smoke-check "${SMOKE_JSON}"

(
  cd "${PKG_DIR}"
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum * | grep -v 'SHA256SUMS.txt' > SHA256SUMS.txt
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 * | grep -v 'SHA256SUMS.txt' > SHA256SUMS.txt
  fi
)

echo "[DONE] Release bundle ready: ${PKG_DIR}"
