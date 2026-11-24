# Port Map — `CodexPlusPlus` (`rust-port/`)

- **Mục tiêu**: Port có kiểm chứng hành vi (`record` -> `diff` -> `gate` -> `app lift`) các phân hệ TypeScript/JavaScript, Python và PowerShell của `CodexPlusPlus` sang Rust trong thư mục biệt lập `rust-port/`, kết hợp tái sử dụng lõi Rust sẵn có (`codex-plus-core`, `codex-plus-data`, `codex-plus-launcher`).
- **Tài liệu liên kết**:
  - Quyết định kiến trúc: [`DECISIONS.md`](./DECISIONS.md) (`accepted`, ADR-001..ADR-007)
  - Từ điển thuật ngữ: [`GLOSSARY.md`](./GLOSSARY.md)
  - Nghiên cứu thay thế AST Babel -> `oxc`: [`research/babel-to-oxc.md`](./research/babel-to-oxc.md)
  - Ranh giới runtime `conversation-canvas`: [`research/canvas-runtime-boundaries.md`](./research/canvas-runtime-boundaries.md) (`ADR-006`)
  - Ranh giới runtime `codex-plus-manager`: [`research/manager-runtime-boundaries.md`](./research/manager-runtime-boundaries.md) (`ADR-002` & `ADR-006`)
  - Báo cáo chuyển giao Checkpoint 4: [`HANDOFF.md`](./HANDOFF.md)
  - Báo cáo phân tích tự động (`analyze@1`): [`analysis/`](./analysis/)
  - Quy trình chuẩn: [`skills/rust-anything/SKILL.md`](./skills/rust-anything/SKILL.md) & [`skills/rust-anything/whole-app.md`](./skills/rust-anything/whole-app.md)

## Tổng quan hiện trạng các phân hệ phi-Rust (từ `rust-anything analyze`)

| Phân hệ nguồn | Ngôn ngữ | Số Unit | Chiến lược đích đã chốt | Trạng thái |
| :--- | :--- | ---: | :--- | :--- |
| `scripts/` (JS/TS + Python) | MJS/CJS/Py | 29 | Pure Rust CLI [`codex-plus-devkit`](./crates/codex-plus-devkit) + helper gates (`i18n-shim-rs`, `release-meta-rs`) | `done (CLI sản xuất + tests)` |
| `assets/native-browser` | JS/MJS | 3 | Pure Rust (`oxc_parser` + `oxc_semantic`) tại [`native-browser-inspector-rs`](./crates/native-browser-inspector-rs) + [`native-browser-inspector.exe`](./crates/codex-plus-devkit/src/bin/native-browser-inspector.rs) | `done (Gate passed 59/59)` |
| `assets/claude-localization` | Py + PS1 + JS | 9 | Công cụ Build/Sign/Inject hợp nhất -> [`claude-patcher-rs`](./crates/claude-patcher-rs); Electron runtime hooks giữ JS gọn (`ADR-006`) | `done (Build/Sign/Inject thuần Rust)` |
| `tools/conversation-canvas` | MJS/JS | 18 | 9 unit WASM (`canvas-wasm/*-rs`) + thư viện hợp nhất [`canvas-core-rs`](./crates/canvas-wasm/canvas-core-rs) & [`canvas-common-rs`](./crates/canvas-wasm/canvas-common-rs); runtime shims giữ JS (`ADR-006`) | `done (9/9 WASM gates + canvas-core-rs)` |
| `tools/codex-wechat` | Python | 1 | Pure Rust CLI [`codex-wechat-rs`](./crates/codex-wechat-rs) (`codex-wechat.exe`) | `done (CLI + mock E2E + differential)` |
| `tools/remote-control-p0` | MJS | 14 | 2 unit WASM (`remote-follower-patches-rs`, `remote-ipc-versions-rs`) + Pure Rust CLI [`remote-control-p0`](./crates/remote-control-rs/remote-control-p0) | `done (2/2 WASM gates + CLI + E2E)` |
| `services/share-site` | JS | 2 | Rust Worker + AES-GCM + Markdown [`share-site-rs`](./crates/share-site-rs) (`known-diffs.jsonl` đóng `POST /api/shares` mở) | `done (host + wasm32 + WebCrypto diff)` |
| `tools/rice-maid-pet` | MJS/JS | 3 | [`rice-maid-pet-rs`](./crates/rice-maid-pet-rs) (Backend, `pre_model_request`, sprite layout math; WebGL/DOM giữ ở JS) | `done (host + wasm32 + contract test)` |
| `apps/codex-plus-manager` | TS/TSX | 161 | 83 Logic units -> Rust/WASM (`manager-models/`: `71` WASM gates + `12` tại `manager-core-rs`); 3 `.manual.{ts,cjs}` blocked; 75 UI/React/Tauri units -> Pure-Rust Dioxus Desktop [`codex-plus-app`](./crates/codex-plus-app) | `done (83/83 logic + Pure-Rust Dioxus GUI)` |
| `assets/inject/renderer-inject` | JS | 56 | Assembler/Verifier tại `codex-plus-devkit`; tính toán thuần & bảo mật (`prompt-args`, `statsig`, `model-catalog`, `picker-groups`, `extension-api`, sanitizers) -> [`renderer-inject-rs`](./crates/renderer-inject-rs) (`wasm32` + JS bridge); 19 DOM shards giữ JS (`ADR-006`) | `done (WASM core + JS bridge + boundaries)` |

## Giai đoạn 1 — Wave 0 (`10/10` Crates `Gate passed.`)

| # | Phân hệ nguồn | Unit nguồn | Crate Rust (`rust-port/crates/`) | Dòng (Rust / Nguồn) | `diff` (khớp) | `gate` (12 checks) | Trạng thái |
| ---: | :--- | :--- | :--- | :--- | ---: | :--- | :--- |
| 1 | `apps/codex-plus-manager` | `src/goals-config.ts` | [`manager-models/goals-config-rs`](./crates/manager-models/goals-config-rs/src/port.rs) | 186 / 88 (2.1x) | 719 / 719 | `Gate passed` | `done (2026-11-24)` |
| 2 | `apps/codex-plus-manager` | `src/code-highlight.ts` | [`manager-models/code-highlight-rs`](./crates/manager-models/code-highlight-rs/src/port.rs) | 546 / 242 (2.3x) | 271 / 271 | `Gate passed` | `done (2026-11-24)` |
| 3 | `apps/codex-plus-manager` | `src/auto-compact.ts` | [`manager-models/auto-compact-rs`](./crates/manager-models/auto-compact-rs/src/port.rs) | 84 / 30 (2.8x) | 880 / 880 | `Gate passed` | `done (2026-11-24)` |
| 4 | `apps/codex-plus-manager` | `src/overview-model.ts` | [`manager-models/overview-model-rs`](./crates/manager-models/overview-model-rs/src/port.rs) | 152 / 49 (3.1x) | 221 / 221 | `Gate passed` | `done (2026-11-24)` |
| 5 | `apps/codex-plus-manager` | `src/provider-name.ts` | [`manager-models/provider-name-rs`](./crates/manager-models/provider-name-rs/src/port.rs) | 158 / 49 (3.2x) | 816 / 816 | `Gate passed` | `done (2026-11-24)` |
| 6 | `apps/codex-plus-manager` | `src/dictation-settings.ts` | [`manager-models/dictation-settings-rs`](./crates/manager-models/dictation-settings-rs/src/port.rs) | 67 / 43 (1.6x) | 212 / 212 | `Gate passed` | `done (2026-11-24)` |
| 7 | `apps/codex-plus-manager` | `src/subscription-policy.ts` | [`manager-models/subscription-policy-rs`](./crates/manager-models/subscription-policy-rs/src/port.rs) | 142 / 43 (3.3x) | 490 / 490 | `Gate passed` | `done (2026-11-24)` |
| 8 | `apps/codex-plus-manager` | `src/whale-settings.ts` | [`manager-models/whale-settings-rs`](./crates/manager-models/whale-settings-rs/src/port.rs) | 164 / 38 (4.3x) | 422 / 422 | `Gate passed` | `done (2026-11-24)` |
| 9 | `apps/codex-plus-manager` | `src/management-model.ts` | [`manager-models/management-model-rs`](./crates/manager-models/management-model-rs/src/port.rs) | 137 / 33 (4.2x) | 887 / 887 | `Gate passed` | `done (2026-11-24)` |
| 10 | `tools/conversation-canvas` | `organize.mjs` | [`canvas-wasm/canvas-organize-rs`](./crates/canvas-wasm/canvas-organize-rs/src/port.rs) | 181 / 45 (4.0x) | 529 / 529 | `Gate passed` | `done (2026-11-24)` |

## Giai đoạn 2 — DevKit CLI, Native Browser AST & Claude Patcher (`Đã hoàn tất Checkpoint 2`)

### 2A. Unit Crates qua `rust-anything gate` (`3/3` Crates `Gate passed.`)

| # | Phân hệ nguồn | Unit nguồn | Crate Rust (`rust-port/crates/`) | Dòng (Rust / Nguồn) | `diff` (khớp) | `gate` (12 checks) | Trạng thái |
| --: | :--- | :--- | :--- | :--- | ---: | :--- | :--- |
| 11 | `assets/native-browser` | `inspect-service.mjs` | [`native-browser-inspector-rs`](./crates/native-browser-inspector-rs/src/port.rs) | 789 / 266 (3.0x) | 59 / 59 | `Gate passed` | `done (2026-11-24)` |
| 12 | `scripts/` | `release-meta.mjs` (pure exports) | [`devkit-units/release-meta-rs`](./crates/devkit-units/release-meta-rs/src/port.rs) | 84 / 118 (0.7x) | 212 / 212 | `Gate passed` | `done (2026-11-24)` |
| 13 | `scripts/` | `provider-editing-i18n-shim.mjs` | [`devkit-units/i18n-shim-rs`](./crates/devkit-units/i18n-shim-rs/src/port.rs) | 65 / 9 (7.2x) | 110 / 110 | `Gate passed` | `done (2026-11-24)` |

### 2B. Standalone Pure-Rust CLI Suites (`Clippy -D warnings` + Differential Tests + Release Build)

| # | Crate CLI Đích | Nguồn hợp nhất | Lệnh / Nhị phân đã kiểm chứng | Trạng thái |
| --: | :--- | :--- | :--- | :--- |
| 14 | [`codex-plus-devkit`](./crates/codex-plus-devkit/Cargo.toml) | `scripts/{release-meta,release,prepare-sherpa-cache,prune-local-builds,relaunch-local,assemble-renderer-inject,split-renderer-inject,assemble-whale-runtime,build-plugin-market-index}.mjs`, `sync_official_models.py`, `assets/native-browser/{inspect-service,require-identification}.mjs` | `codex-plus-devkit.exe`, `native-browser-inspector.exe` — 10/10 differential tests PASS (so khớp trực tiếp Node/Python), live `--check`/`--dry-run` PASS | `done (2026-11-24)` |
| 15 | [`claude-patcher-rs`](./crates/claude-patcher-rs/Cargo.toml) | `assets/claude-localization/{build-copy.py,build-copy.ps1,sign-copy.py,inject-cn.ps1,inject-cn.js}` | `claude-patcher.exe` (`build-copy`, `sign-copy`, `inject-cn`) — 3/3 differential Python + mock CDP WebSocket E2E tests PASS, [`known-diffs.jsonl`](./crates/claude-patcher-rs/known-diffs.jsonl) ghi 3 bản vá chủ đích (`ADR-007`) | `done (2026-11-24)` |

**Tổng kết tích lũy (Giai đoạn 1 + 2)**: **`13/13`** crates vượt đầy đủ `rust-anything gate` (`5,828/5,828` ca hành vi khớp 100%) + **`2`** bộ công cụ CLI Pure-Rust đa nền tảng (`13/13` test đối chiếu & E2E PASS), hoàn toàn cô lập trong `rust-port/`.

## Giai đoạn 3 — Tools & Services (`Đã hoàn tất Checkpoint 3`)

### 3A. Unit Crates qua `rust-anything gate` (`10/10` Crates mới `Gate passed.`)

| # | Phân hệ nguồn | Unit nguồn | Crate Rust (`rust-port/crates/`) | Dòng (Rust / Nguồn) | `diff` (khớp) | `gate` (12 checks) | Trạng thái |
| --: | :--- | :--- | :--- | :--- | ---: | :--- | :--- |
| 16 | `tools/conversation-canvas` | `native-history.mjs` (`isExportSizeError`) | [`canvas-wasm/canvas-native-history-rs`](./crates/canvas-wasm/canvas-native-history-rs/src/port.rs) | 74 / 117 (0.6x) | 14 / 14 | `Gate passed` | `done (2026-11-24)` |
| 17 | `tools/conversation-canvas` | `model.mjs` (`parseMessage`, `excerpt`, `buildGraph`, `visibleTreeRows`) | [`canvas-wasm/canvas-model-rs`](./crates/canvas-wasm/canvas-model-rs/src/port.rs) | 18 / 54 (0.3x) | 615 / 615 | `Gate passed` | `done (2026-11-24)` |
| 18 | `tools/conversation-canvas` | `bridge-data.mjs` (`graphFromExport`) | [`canvas-wasm/canvas-bridge-data-rs`](./crates/canvas-wasm/canvas-bridge-data-rs/src/port.rs) | 76 / 71 (1.1x) | 212 / 212 | `Gate passed` | `done (2026-11-24)` |
| 19 | `tools/conversation-canvas` | `tree-markup.mjs` (`treeMarkup`) | [`canvas-wasm/canvas-tree-markup-rs`](./crates/canvas-wasm/canvas-tree-markup-rs/src/port.rs) | 125 / 63 (2.0x) | 212 / 212 | `Gate passed` | `done (2026-11-24)` |
| 20 | `tools/conversation-canvas` | `external-api.mjs` (`apiEndpoint`, `storedApiConfig`, `apiRequestBody`, `completionText`) | [`canvas-wasm/canvas-external-api-rs`](./crates/canvas-wasm/canvas-external-api-rs/src/port.rs) | 118 / 127 (0.9x) | 158 / 158 | `Gate passed` | `done (2026-11-24)` |
| 21 | `tools/conversation-canvas` | `view-pages.mjs` (`transcriptPage`) | [`canvas-wasm/canvas-view-pages-rs`](./crates/canvas-wasm/canvas-view-pages-rs/src/port.rs) | 5 / 15 (0.3x) | 111 / 111 | `Gate passed` | `done (2026-11-24)` |
| 22 | `tools/conversation-canvas` | `tree-canvas.mjs` (`fitCanvas`, `zoomCanvas`, `canvasViewport`, `canvasCardsMarkup`) | [`canvas-wasm/canvas-tree-canvas-rs`](./crates/canvas-wasm/canvas-tree-canvas-rs/src/port.rs) | 217 / 89 (2.4x) | 871 / 871 | `Gate passed` | `done (2026-11-24)` |
| 23 | `tools/conversation-canvas` | `long-organizer.mjs` (`nextBatch`, `selectTreeContext`, `batchPrompt`) | [`canvas-wasm/canvas-long-organizer-rs`](./crates/canvas-wasm/canvas-long-organizer-rs/src/port.rs) | 305 / 186 (1.6x) | 966 / 966 | `Gate passed` | `done (2026-11-24)` |
| 24 | `tools/remote-control-p0` | `patches` (`applyPatches`) | [`remote-control-rs/remote-follower-patches-rs`](./crates/remote-control-rs/remote-follower-patches-rs/src/port.rs) | 102 / 110 (0.9x) | 93 / 93 | `Gate passed` | `done (2026-11-24)` |
| 25 | `tools/remote-control-p0` | `ipc-versions.mjs` (`extractIpcVersions`) | [`remote-control-rs/remote-ipc-versions-rs`](./crates/remote-control-rs/remote-ipc-versions-rs/src/port.rs) | 32 / 36 (0.9x) | 17 / 17 | `Gate passed` | `done (2026-11-24)` |

### 3B. Thư viện hợp nhất, Service Worker & CLI (`fmt` + `Clippy -D warnings` + Tests + `wasm32`/Release Build)

| # | Crate Đích (`rust-port/crates/`) | Nguồn hợp nhất / Tài liệu | Kết quả kiểm chứng | Trạng thái |
| --: | :--- | :--- | :--- | :--- |
| 26 | [`canvas-wasm/canvas-common-rs`](./crates/canvas-wasm/canvas-common-rs/Cargo.toml) + [`canvas-wasm/canvas-core-rs`](./crates/canvas-wasm/canvas-core-rs/Cargo.toml) | Hợp nhất toàn bộ `conversation-canvas` (`task_tree_view`, `layout_task_canvas`, `prepare_history`, `merge_batch`, `SseStreamDecoder`, `read_native_history_with`, `bundle_userscript`) + [`canvas-runtime-boundaries.md`](./research/canvas-runtime-boundaries.md) | `fmt` + `clippy -D warnings` PASS; `3/3` differential tests vs Node + `public/canvas.user.js` PASS; `wasm32-unknown-unknown` PASS | `done (2026-11-24)` |
| 27 | [`codex-wechat-rs`](./crates/codex-wechat-rs/Cargo.toml) | `tools/codex-wechat/codex_wechat.py` (`run`, `login`) | `codex-wechat.exe` (`9,834,817` bytes) — `fmt` + `clippy -D warnings` PASS; `3/3` Python differential & mock app-server/QR E2E PASS; `--help` PASS | `done (2026-11-24)` |
| 28 | [`remote-control-rs/remote-control-p0`](./crates/remote-control-rs/remote-control-p0/Cargo.toml) | `tools/remote-control-p0` (`asar`, `ipc_client`, `follower`, `sidecar`, `mock_responses`, `runners`) | `remote-control-p0.exe` (`10,744,808` bytes) — `fmt` + `clippy -D warnings` PASS; `3/3` Node ASAR diff, HTTP/SSE mock-responses & IPC/Sidecar E2E PASS; `--help` PASS | `done (2026-11-24)` |
| 29 | [`share-site-rs`](./crates/share-site-rs/Cargo.toml) | `services/share-site/{_worker.js,app.js}` + [`known-diffs.jsonl`](./crates/share-site-rs/known-diffs.jsonl) | `fmt` + `clippy -D warnings` PASS; `3/3` tests (Node WebCrypto AES-256-GCM cross-verify, `vm` Markdown diff, Worker route security gate `POST /api/shares` -> HTTP 403) PASS; `wasm32-unknown-unknown` PASS | `done (2026-11-24)` |
| 30 | [`rice-maid-pet-rs`](./crates/rice-maid-pet-rs/Cargo.toml) | `tools/rice-maid-pet/{backend/main.mjs,hooks/request.mjs}` + sprite layout math | `fmt` + `clippy -D warnings` PASS; `1/1` contract test PASS; `wasm32-unknown-unknown` PASS | `done (2026-11-24)` |

**Tổng kết tích lũy (Giai đoạn 1 + 2 + 3)**: **`23/23`** crate unit vượt đầy đủ `rust-anything gate` (`9,097/9,097` ca hành vi khớp 100%) + **`8`** crate ứng dụng/thư viện hợp nhất (`codex-plus-devkit`, `claude-patcher-rs`, `canvas-common-rs`, `canvas-core-rs`, `codex-wechat-rs`, `remote-control-p0`, `share-site-rs`, `rice-maid-pet-rs`) PASS 100% kiểm thử (`26/26` tests), build release & `wasm32-unknown-unknown`, hoàn toàn cô lập trong `rust-port/`.

## Giai đoạn 4 — Manager Domain Logic & Models (`Đã hoàn tất Checkpoint 4`)

### 4A. Unit Crates qua `rust-anything gate` (`62/62` Crates mới `Gate passed.`, nâng `manager-models/` lên `71/71` và toàn repo lên `85/85`)

| # | Phân hệ nguồn | Unit nguồn | Crate Rust (`rust-port/crates/`) | Dòng (Rust / Nguồn) | `diff` (khớp) | `gate` (12 checks) | Trạng thái |
| --: | :--- | :--- | :--- | :--- | ---: | :--- | :--- |
| 31 | `apps/codex-plus-manager` | `src/agent-cache-model.ts` | [`manager-models/agent-cache-model-rs`](./crates/manager-models/agent-cache-model-rs/src/port.rs) | 145 / 56 (2.6x) | 1024/1024 | `Gate passed` | `done (2026-11-24)` |
| 32 | `apps/codex-plus-manager` | `src/agent-collaboration-queue.ts` | [`manager-models/agent-collaboration-queue-rs`](./crates/manager-models/agent-collaboration-queue-rs/src/port.rs) | 81 / 34 (2.4x) | 333/333 | `Gate passed` | `done (2026-11-24)` |
| 33 | `apps/codex-plus-manager` | `src/agent-dashboard-model.ts` | [`manager-models/agent-dashboard-model-rs`](./crates/manager-models/agent-dashboard-model-rs/src/port.rs) | 274 / 2664 (0.1x) | 669/669 | `Gate passed` | `done (2026-11-24)` |
| 34 | `apps/codex-plus-manager` | `src/agent-focus-model.ts` | [`manager-models/agent-focus-model-rs`](./crates/manager-models/agent-focus-model-rs/src/port.rs) | 108 / 51 (2.1x) | 922/922 | `Gate passed` | `done (2026-11-24)` |
| 35 | `apps/codex-plus-manager` | `src/agent-mcp-model.ts` | [`manager-models/agent-mcp-model-rs`](./crates/manager-models/agent-mcp-model-rs/src/port.rs) | 164 / 54 (3.0x) | 1335/1335 | `Gate passed` | `done (2026-11-24)` |
| 36 | `apps/codex-plus-manager` | `src/aggregate-members.ts` | [`manager-models/aggregate-members-rs`](./crates/manager-models/aggregate-members-rs/src/port.rs) | 198 / 72 (2.8x) | 1120/1120 | `Gate passed` | `done (2026-11-24)` |
| 37 | `apps/codex-plus-manager` | `src/aggregate-routes.ts` | [`manager-models/aggregate-routes-rs`](./crates/manager-models/aggregate-routes-rs/src/port.rs) | 167 / 61 (2.7x) | 461/461 | `Gate passed` | `done (2026-11-24)` |
| 38 | `apps/codex-plus-manager` | `src/config-doctor-model.ts` | [`manager-models/config-doctor-model-rs`](./crates/manager-models/config-doctor-model-rs/src/port.rs) | 544 / 195 (2.8x) | 4028/4028 | `Gate passed` | `done (2026-11-24)` |
| 39 | `apps/codex-plus-manager` | `src/context-config-text.ts` | [`manager-models/context-config-text-rs`](./crates/manager-models/context-config-text-rs/src/port.rs) | 475 / 235 (2.0x) | 3629/3629 | `Gate passed` | `done (2026-11-24)` |
| 40 | `apps/codex-plus-manager` | `src/dream-skin.ts` | [`manager-models/dream-skin-rs`](./crates/manager-models/dream-skin-rs/src/port.rs) | 380 / 333 (1.1x) | 858/858 | `Gate passed` | `done (2026-11-24)` |
| 41 | `apps/codex-plus-manager` | `src/enhancement-catalog.ts` | [`manager-models/enhancement-catalog-rs`](./crates/manager-models/enhancement-catalog-rs/src/port.rs) | 311 / 653 (0.5x) | 1690/1690 | `Gate passed` | `done (2026-11-24)` |
| 42 | `apps/codex-plus-manager` | `src/enhancement-navigation.ts` | [`manager-models/enhancement-navigation-rs`](./crates/manager-models/enhancement-navigation-rs/src/port.rs) | 33 / 11 (3.0x) | 214/214 | `Gate passed` | `done (2026-11-24)` |
| 43 | `apps/codex-plus-manager` | `src/extension-config-schema.ts` | [`manager-models/extension-config-schema-rs`](./crates/manager-models/extension-config-schema-rs/src/port.rs) | 209 / 65 (3.2x) | 208/208 | `Gate passed` | `done (2026-11-24)` |
| 44 | `apps/codex-plus-manager` | `src/extension-host-model.ts` | [`manager-models/extension-host-model-rs`](./crates/manager-models/extension-host-model-rs/src/port.rs) | 137 / 102 (1.3x) | 901/901 | `Gate passed` | `done (2026-11-24)` |
| 45 | `apps/codex-plus-manager` | `src/extension-manager-bridge.ts` | [`manager-models/extension-manager-bridge-rs`](./crates/manager-models/extension-manager-bridge-rs/src/port.rs) | 92 / 141 (0.7x) | 262/262 | `Gate passed` | `done (2026-11-24)` |
| 46 | `apps/codex-plus-manager` | `src/extension-manager-pages.ts` | [`manager-models/extension-manager-pages-rs`](./crates/manager-models/extension-manager-pages-rs/src/port.rs) | 285 / 103 (2.8x) | 885/885 | `Gate passed` | `done (2026-11-24)` |
| 47 | `apps/codex-plus-manager` | `src/gateway-agent-safety.ts` | [`manager-models/gateway-agent-safety-rs`](./crates/manager-models/gateway-agent-safety-rs/src/port.rs) | 271 / 103 (2.6x) | 1193/1193 | `Gate passed` | `done (2026-11-24)` |
| 48 | `apps/codex-plus-manager` | `src/gateway-auto-compact-model.ts` | [`manager-models/gateway-auto-compact-model-rs`](./crates/manager-models/gateway-auto-compact-model-rs/src/port.rs) | 101 / 20 (5.0x) | 459/459 | `Gate passed` | `done (2026-11-24)` |
| 49 | `apps/codex-plus-manager` | `src/gateway-model.ts` | [`manager-models/gateway-model-rs`](./crates/manager-models/gateway-model-rs/src/port.rs) | 3300 / 2610 (1.3x) | 8602/8602 | `Gate passed` | `done (2026-11-24)` |
| 50 | `apps/codex-plus-manager` | `src/github-repository.ts` | [`manager-models/github-repository-rs`](./crates/manager-models/github-repository-rs/src/port.rs) | 128 / 73 (1.8x) | 257/257 | `Gate passed` | `done (2026-11-24)` |
| 51 | `apps/codex-plus-manager` | `src/history-recovery-report.ts` | [`manager-models/history-recovery-report-rs`](./crates/manager-models/history-recovery-report-rs/src/port.rs) | 43 / 15 (2.9x) | 206/206 | `Gate passed` | `done (2026-11-24)` |
| 52 | `apps/codex-plus-manager` | `src/hub-overview-model.ts` | [`manager-models/hub-overview-model-rs`](./crates/manager-models/hub-overview-model-rs/src/port.rs) | 567 / 2871 (0.2x) | 1428/1428 | `Gate passed` | `done (2026-11-24)` |
| 53 | `apps/codex-plus-manager` | `src/i18n.ts` | [`manager-models/i18n-rs`](./crates/manager-models/i18n-rs/src/port.rs) | 141 / 47 (3.0x) | 710/710 | `Gate passed` | `done (2026-11-24)` |
| 54 | `apps/codex-plus-manager` | `src/launch-status.ts` | [`manager-models/launch-status-rs`](./crates/manager-models/launch-status-rs/src/port.rs) | 91 / 28 (3.2x) | 432/432 | `Gate passed` | `done (2026-11-24)` |
| 55 | `apps/codex-plus-manager` | `src/log-view-model.ts` | [`manager-models/log-view-model-rs`](./crates/manager-models/log-view-model-rs/src/port.rs) | 479 / 158 (3.0x) | 1629/1629 | `Gate passed` | `done (2026-11-24)` |
| 56 | `apps/codex-plus-manager` | `src/model-groups.ts` | [`manager-models/model-groups-rs`](./crates/manager-models/model-groups-rs/src/port.rs) | 193 / 52 (3.7x) | 444/444 | `Gate passed` | `done (2026-11-24)` |
| 57 | `apps/codex-plus-manager` | `src/model-metadata.ts` | [`manager-models/model-metadata-rs`](./crates/manager-models/model-metadata-rs/src/port.rs) | 1896 / 819 (2.3x) | 8399/8399 | `Gate passed` | `done (2026-11-24)` |
| 58 | `apps/codex-plus-manager` | `src/model-reasoning.ts` | [`manager-models/model-reasoning-rs`](./crates/manager-models/model-reasoning-rs/src/port.rs) | 365 / 951 (0.4x) | 1600/1600 | `Gate passed` | `done (2026-11-24)` |
| 59 | `apps/codex-plus-manager` | `src/model-routes.ts` | [`manager-models/model-routes-rs`](./crates/manager-models/model-routes-rs/src/port.rs) | 392 / 161 (2.4x) | 1141/1141 | `Gate passed` | `done (2026-11-24)` |
| 60 | `apps/codex-plus-manager` | `src/model-windows.ts` | [`manager-models/model-windows-rs`](./crates/manager-models/model-windows-rs/src/port.rs) | 677 / 1044 (0.6x) | 2551/2551 | `Gate passed` | `done (2026-11-24)` |
| 61 | `apps/codex-plus-manager` | `src/multi-launch-model.ts` | [`manager-models/multi-launch-model-rs`](./crates/manager-models/multi-launch-model-rs/src/port.rs) | 189 / 65 (2.9x) | 1856/1856 | `Gate passed` | `done (2026-11-24)` |
| 62 | `apps/codex-plus-manager` | `src/native-auth-model.ts` | [`manager-models/native-auth-model-rs`](./crates/manager-models/native-auth-model-rs/src/port.rs) | 95 / 28 (3.4x) | 410/410 | `Gate passed` | `done (2026-11-24)` |
| 63 | `apps/codex-plus-manager` | `src/notice-model.ts` | [`manager-models/notice-model-rs`](./crates/manager-models/notice-model-rs/src/port.rs) | 43 / 15 (2.9x) | 465/465 | `Gate passed` | `done (2026-11-24)` |
| 64 | `apps/codex-plus-manager` | `src/onboarding-model.ts` | [`manager-models/onboarding-model-rs`](./crates/manager-models/onboarding-model-rs/src/port.rs) | 225 / 104 (2.2x) | 639/639 | `Gate passed` | `done (2026-11-24)` |
| 65 | `apps/codex-plus-manager` | `src/onboarding-setup-model.ts` | [`manager-models/onboarding-setup-model-rs`](./crates/manager-models/onboarding-setup-model-rs/src/port.rs) | 654 / 2845 (0.2x) | 919/919 | `Gate passed` | `done (2026-11-24)` |
| 66 | `apps/codex-plus-manager` | `src/page-groups.ts` | [`manager-models/page-groups-rs`](./crates/manager-models/page-groups-rs/src/port.rs) | 70 / 18 (3.9x) | 458/458 | `Gate passed` | `done (2026-11-24)` |
| 67 | `apps/codex-plus-manager` | `src/plugin-market-model.ts` | [`manager-models/plugin-market-model-rs`](./crates/manager-models/plugin-market-model-rs/src/port.rs) | 402 / 119 (3.4x) | 1390/1390 | `Gate passed` | `done (2026-11-24)` |
| 68 | `apps/codex-plus-manager` | `src/popover-layer.ts` | [`manager-models/popover-layer-rs`](./crates/manager-models/popover-layer-rs/src/port.rs) | 253 / 132 (1.9x) | 637/637 | `Gate passed` | `done (2026-11-24)` |
| 69 | `apps/codex-plus-manager` | `src/provider-api-keys.ts` | [`manager-models/provider-api-keys-rs`](./crates/manager-models/provider-api-keys-rs/src/port.rs) | 95 / 75 (1.3x) | 474/474 | `Gate passed` | `done (2026-11-24)` |
| 70 | `apps/codex-plus-manager` | `src/provider-balances.ts` | [`manager-models/provider-balances-rs`](./crates/manager-models/provider-balances-rs/src/port.rs) | 112 / 49 (2.3x) | 437/437 | `Gate passed` | `done (2026-11-24)` |
| 71 | `apps/codex-plus-manager` | `src/provider-brands.ts` | [`manager-models/provider-brands-rs`](./crates/manager-models/provider-brands-rs/src/port.rs) | 168 / 66 (2.5x) | 494/494 | `Gate passed` | `done (2026-11-24)` |
| 72 | `apps/codex-plus-manager` | `src/provider-form-model.ts` | [`manager-models/provider-form-model-rs`](./crates/manager-models/provider-form-model-rs/src/port.rs) | 337 / 2690 (0.1x) | 2994/2994 | `Gate passed` | `done (2026-11-24)` |
| 73 | `apps/codex-plus-manager` | `src/provider-meta-sync-model.ts` | [`manager-models/provider-meta-sync-model-rs`](./crates/manager-models/provider-meta-sync-model-rs/src/port.rs) | 297 / 2738 (0.1x) | 1754/1754 | `Gate passed` | `done (2026-11-24)` |
| 74 | `apps/codex-plus-manager` | `src/provider-pool-layout.ts` | [`manager-models/provider-pool-layout-rs`](./crates/manager-models/provider-pool-layout-rs/src/port.rs) | 253 / 2627 (0.1x) | 439/439 | `Gate passed` | `done (2026-11-24)` |
| 75 | `apps/codex-plus-manager` | `src/provider-sync-flow.ts` | [`manager-models/provider-sync-flow-rs`](./crates/manager-models/provider-sync-flow-rs/src/port.rs) | 122 / 63 (1.9x) | 421/421 | `Gate passed` | `done (2026-11-24)` |
| 76 | `apps/codex-plus-manager` | `src/provider-sync-model.ts` | [`manager-models/provider-sync-model-rs`](./crates/manager-models/provider-sync-model-rs/src/port.rs) | 193 / 52 (3.7x) | 1106/1106 | `Gate passed` | `done (2026-11-24)` |
| 77 | `apps/codex-plus-manager` | `src/provider-sync-target.ts` | [`manager-models/provider-sync-target-rs`](./crates/manager-models/provider-sync-target-rs/src/port.rs) | 86 / 21 (4.1x) | 445/445 | `Gate passed` | `done (2026-11-24)` |
| 78 | `apps/codex-plus-manager` | `src/provider-test.ts` | [`manager-models/provider-test-rs`](./crates/manager-models/provider-test-rs/src/port.rs) | 143 / 64 (2.2x) | 475/475 | `Gate passed` | `done (2026-11-24)` |
| 79 | `apps/codex-plus-manager` | `src/relay-billing.ts` | [`manager-models/relay-billing-rs`](./crates/manager-models/relay-billing-rs/src/port.rs) | 558 / 182 (3.1x) | 2378/2378 (`7` [`known-diffs`](./crates/manager-models/relay-billing-rs/known-diffs.jsonl)) | `Gate passed` | `done (2026-11-24)` |
| 80 | `apps/codex-plus-manager` | `src/relay-headers.ts` | [`manager-models/relay-headers-rs`](./crates/manager-models/relay-headers-rs/src/port.rs) | 240 / 80 (3.0x) | 1862/1862 | `Gate passed` | `done (2026-11-24)` |
| 81 | `apps/codex-plus-manager` | `src/relay-live-files.ts` | [`manager-models/relay-live-files-rs`](./crates/manager-models/relay-live-files-rs/src/port.rs) | 50 / 17 (2.9x) | 459/459 | `Gate passed` | `done (2026-11-24)` |
| 82 | `apps/codex-plus-manager` | `src/relay-profile-editing.ts` | [`manager-models/relay-profile-editing-rs`](./crates/manager-models/relay-profile-editing-rs/src/port.rs) | 3028 / 2621 (1.2x) | 13535/13535 | `Gate passed` | `done (2026-11-24)` |
| 83 | `apps/codex-plus-manager` | `src/relay-session.ts` | [`manager-models/relay-session-rs`](./crates/manager-models/relay-session-rs/src/port.rs) | 43 / 15 (2.9x) | 480/480 | `Gate passed` | `done (2026-11-24)` |
| 84 | `apps/codex-plus-manager` | `src/request-flow-model.ts` | [`manager-models/request-flow-model-rs`](./crates/manager-models/request-flow-model-rs/src/port.rs) | 340 / 2727 (0.1x) | 1924/1924 | `Gate passed` | `done (2026-11-24)` |
| 85 | `apps/codex-plus-manager` | `src/script-market-model.ts` | [`manager-models/script-market-model-rs`](./crates/manager-models/script-market-model-rs/src/port.rs) | 272 / 75 (3.6x) | 1089/1089 | `Gate passed` | `done (2026-11-24)` |
| 86 | `apps/codex-plus-manager` | `src/settings-rebase.ts` | [`manager-models/settings-rebase-rs`](./crates/manager-models/settings-rebase-rs/src/port.rs) | 193 / 51 (3.8x) | 438/438 | `Gate passed` | `done (2026-11-24)` |
| 87 | `apps/codex-plus-manager` | `src/skill-agent-batch.ts` | [`manager-models/skill-agent-batch-rs`](./crates/manager-models/skill-agent-batch-rs/src/port.rs) | 198 / 140 (1.4x) | 1369/1369 | `Gate passed` | `done (2026-11-24)` |
| 88 | `apps/codex-plus-manager` | `src/skill-groups.ts` | [`manager-models/skill-groups-rs`](./crates/manager-models/skill-groups-rs/src/port.rs) | 845 / 271 (3.1x) | 3295/3295 | `Gate passed` | `done (2026-11-24)` |
| 89 | `apps/codex-plus-manager` | `src/skill-repository.ts` | [`manager-models/skill-repository-rs`](./crates/manager-models/skill-repository-rs/src/port.rs) | 138 / 91 (1.5x) | 270/270 | `Gate passed` | `done (2026-11-24)` |
| 90 | `apps/codex-plus-manager` | `src/theme-mode.ts` | [`manager-models/theme-mode-rs`](./crates/manager-models/theme-mode-rs/src/port.rs) | 69 / 50 (1.4x) | 646/646 | `Gate passed` | `done (2026-11-24)` |
| 91 | `apps/codex-plus-manager` | `src/tray-panel-model.ts` | [`manager-models/tray-panel-model-rs`](./crates/manager-models/tray-panel-model-rs/src/port.rs) | 485 / 183 (2.7x) | 1910/1910 | `Gate passed` | `done (2026-11-24)` |
| 92 | `apps/codex-plus-manager` | `src/update-notification.ts` | [`manager-models/update-notification-rs`](./crates/manager-models/update-notification-rs/src/port.rs) | 129 / 92 (1.4x) | 476/476 | `Gate passed` | `done (2026-11-24)` |

> Ghi chú: `i18n-rs` và `relay-profile-editing-rs` chạy diff/gate với Node 22 ESM hook [`manager-models/ts-loader.mjs`](./crates/manager-models/ts-loader.mjs) (phân giải `@/*` và import `.ts` không đuôi). Toàn bộ `61/62` crate có `0` ngoại lệ `known-diffs.jsonl`; riêng `relay-billing-rs` ghi `7` ngoại lệ phản chiếu thuộc tính `.at` kiểu hàm trên đầu vào không phải object.

### 4B. Thư viện nền & Hợp nhất Domain Manager (`fmt` + `Clippy -D warnings` + Tests + `wasm32`)

| # | Crate Đích (`rust-port/crates/`) | Nguồn hợp nhất / Tài liệu | Kết quả kiểm chứng | Trạng thái |
| --: | :--- | :--- | :--- | :--- |
| 93 | [`manager-models/manager-common-rs`](./crates/manager-models/manager-common-rs/Cargo.toml) | Assets nhúng (`presets.json`, `i18n-en.json`) + `js_rt`, `domain`, `gateway_shared`, `relay_shared` | `fmt --check` + `clippy -D warnings` PASS; `1/1` test (`embedded_assets_load_completely`) PASS; `wasm32-unknown-unknown` PASS | `done (2026-11-24)` |
| 94 | [`manager-models/manager-core-rs`](./crates/manager-models/manager-core-rs/Cargo.toml) | Hợp nhất `12` unit tĩnh/DOM-logic (`presets`, `i18n-en`, `mcp-presets`, `vlm-test-translation`, `flow-provider-colors`, `skill-source-scan`, `escape-layer`, `mac-titlebar`, `provider-brand-icons`, `settings-events`, `lib/utils`, `legacy-webkit`), các hàm `js_only` thuần Rust (`js_only_bridges`), `80` hàm `relay_editing` + [`manager-runtime-boundaries.md`](./research/manager-runtime-boundaries.md) | `fmt --check` + `clippy -D warnings` PASS; `4/4` integration tests (Node 22 TS oracle + [`42-provider-editing.generated.js`](../assets/inject/renderer-inject/42-provider-editing.generated.js) `vm`) PASS; `wasm32-unknown-unknown` PASS | `done (2026-11-24)` |

## Giai đoạn 5 — renderer-inject WASM, Vá Bảo Mật & Ứng Dụng Desktop `codex-plus-app` (Đã hoàn tất Checkpoint 5)

| # | Crate Đích (`rust-port/crates/`) | Nguồn hợp nhất / Tài liệu | Kết quả kiểm chứng | Trạng thái |
| --: | :--- | :--- | :--- | :--- |
| 95 | [`renderer-inject-rs`](./crates/renderer-inject-rs/Cargo.toml) | `96-prompt-args.js`, `34-statsig-overrides.js`, `70-model-catalog.js`, `71-model-picker-groups.js`, `91-extension-api.js` (25-route allowlist), XSS/CSS/SVG/URL sanitizers (`C-3`/`H-5`) + [`renderer-inject-wasm-bridge.js`](./crates/renderer-inject-rs/bridge/renderer-inject-wasm-bridge.js) + [`renderer-inject-boundaries.md`](./research/renderer-inject-boundaries.md) (`1,668` dòng Rust) | `fmt --check` + `clippy -D warnings` PASS; `3/3` differential tests (Node 22 `vm`) PASS; `wasm32-unknown-unknown --release` + `wasm-bindgen --target web` PASS | `done (2026-11-24)` |
| 96 | [`codex-plus-app/sherpa-onnx-gnu-shim`](./crates/codex-plus-app/sherpa-onnx-gnu-shim/Cargo.toml) | Shim thuần Rust `#![forbid(unsafe_code)]` (`v1.13.8`) thay thế `.lib` MSVC `/MT` của `sherpa-onnx` trên `x86_64-pc-windows-gnu` (`ADR-003`) | `fmt --check` + `clippy -D warnings` PASS; `1/1` test PASS | `done (2026-11-24)` |
| 97 | [`codex-plus-app`](./crates/codex-plus-app/Cargo.toml) | Ứng dụng Desktop Pure-Rust (`dioxus-desktop 0.7`, `gnu` feature, `#![forbid(unsafe_code)]`, `2,637` dòng Rust): 32 route Manager, nhúng 27 sheet CSS gốc, kết nối in-process `codex-plus-core` + `codex-plus-data` + `codex-plus-launcher` + `manager-core-rs` + `renderer-inject-rs`, vá bảo mật `C-1..C-4` & `H-4.1.6`/`H-4.2.2..4.2.4` ([`known-diffs.jsonl`](./crates/codex-plus-app/known-diffs.jsonl)) | `fmt --check` + `clippy --no-deps -D warnings` PASS; `6/6` security & smoke tests PASS; `codex-plus-app.exe` (`111.8 MB`) + `WebView2Loader.dll` (`160 KB`); `--smoke-check` `ok: true` (`HTTP 200` Gateway `:57321`, Codex `26.1117.4957.0`, `40` phiên); GUI live `Responding=True`, `WorkingSet=32.5 MB` | `done (2026-11-24)` |

**Tổng kết tích lũy (Giai đoạn 1 → 6 — Hoàn tất 100%)**: **`98` Cargo crates** (`535` tệp `.rs`, **`144,094` dòng mã Rust** mới + **`393,001` dòng Rust** lõi gốc) gồm **`85/85`** crate unit vượt đầy đủ `rust-anything gate` (**`102,631`** ca hành vi khớp) + **`13`** crate ứng dụng/thư viện hợp nhất & GNU shim PASS 100% kiểm thử (**`48/48` tests** ngoài gate), `0%` fallback rate, hoàn toàn cô lập trong `rust-port/`.

## Giai đoạn 6 — Idiomatic Rust Lift, Benchmark & Bàn Giao Cutover (Đã hoàn tất Checkpoint 6)

| Hạng mục | Thành phần đích (`rust-port/`) | Kết quả kiểm chứng | Trạng thái |
| :--- | :--- | :--- | :--- |
| 1. Idiomatic Rust Lift (`lifted.rs`) | [`manager-core-rs/src/lifted.rs`](./crates/manager-models/manager-core-rs/src/lifted.rs) + tích hợp tô màu cú pháp zero-copy & KPI kiểu mạnh vào [`codex-plus-app`](./crates/codex-plus-app/Cargo.toml) | `4/4` `lifted_equivalence_tests` PASS; tăng tốc `--release`: **`26.68x`** (`tokenize_code`), **`51.54x`** (`overview`), **`3,092.34x`** (`toml_section_lookup`) | `done (2026-11-24)` |
| 2. Pure-Rust Devkit & Whole-App Replay | `relaunch-local --pure-rust` trong [`codex-plus-devkit`](./crates/codex-plus-devkit/Cargo.toml) + [`whole_app_replay_tests.rs`](./crates/codex-plus-devkit/tests/whole_app_replay_tests.rs) | `13/13` devkit & replay tests PASS; xác nhận `0` kill switch & `0` tiến trình `node`/`python` trong mã sản xuất | `done (2026-11-24)` |
| 3. Báo cáo Nghiệm thu & Cutover Wizard | [`REPORT.md`](./REPORT.md), [`cutover.ps1`](./cutover.ps1), [`cutover.sh`](./cutover.sh) | `cutover.ps1 -Check` PASS trên toàn bộ 7 tệp thực thi release (`0` lỗi, Gateway `:57321` `HTTP 200`) | `done (2026-11-24)` |
