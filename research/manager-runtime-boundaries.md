# `codex-plus-manager` Runtime Boundaries (`ADR-002` & `ADR-006`)

Theo [`DECISIONS.md`](../DECISIONS.md), toàn bộ tầng domain logic của `apps/codex-plus-manager` đã được tách sạch khỏi React/DOM/Tauri shell trong Giai đoạn 1 và Giai đoạn 4 (`83/83` logic/model units = `100%` tập `76 ready` + `7 caution` trong [`analysis/manager-frontend.md`](../analysis/manager-frontend.md)).

## 1. Phân bố `161` units trong `apps/codex-plus-manager`

- **`71` crate WASM qua `rust-anything gate`** (đạt `12/12` checks):
  - `9` crate từ Giai đoạn 1 (`account-switch-rules-rs`, `custom-headers-rs`, `gateway-agents-rs`, `model-list-rs`, `oauth-browser-rs`, `provider-capabilities-rs`, `relay-mode-rs`, `session-providers-rs`, `version-sort-rs`).
  - `62` crate mới ở Giai đoạn 4 phủ trọn các Wave 0, 1, 2, 3, 4 và 13.
- **`12` unit tĩnh / DOM-logic / dữ liệu lớn** port thuần Rust tại [`manager-core-rs`](../crates/manager-models/manager-core-rs/src/static_units.rs) (dùng chung tài nguyên nhúng trong [`manager-common-rs`](../crates/manager-models/manager-common-rs/src/assets_data.rs)):
  1. `presets.ts` (`presets()`) — 23 preset cấu hình provider nhúng từ `assets/presets.json`.
  2. `i18n-en.ts` (`en_dict()`, `translate_en()`, `translate_template_en()`) — 3.597 plain keys, 625 template keys, 172 backend keys và 112 backend regex patterns (`fancy-regex`) nhúng từ `assets/i18n-en.json`.
  3. `mcp-presets.ts` (`mcp_presets()`, `mcp_preset_by_id()`, `mcp_preset_toml_body()`) — 5 MCP preset chuẩn và sinh TOML theo biến thể `unix`/`windows`.
  4. `vlm-test-translation.ts` (`vlm_test_translation()`, `vlm_test_translation_zh()`) — chuẩn hóa chuỗi kết quả test VLM với callback dịch thuật.
  5. `flow-provider-colors.ts` (`FlowProviderColorStore`, `assign_provider_slot()`) — quản lý slot màu `0..2` và fallback `3` (tách khỏi `localStorage` bên trình duyệt).
  6. `skill-source-scan.ts` (`SkillSourceScanState`, `skill_source_repo_key()`) — state machine quét repo skill (không kéo runtime `@tauri-apps/api`).
  7. `escape-layer.ts` (`EscapeLayerStack`) — stack đăng ký/đóng modal/popover theo LIFO khi nhấn phím `Escape`.
  8. `mac-titlebar.ts` (`uses_mac_overlay_titlebar()`) — logic phát hiện overlay titlebar từ URL query, cờ Tauri và platform/userAgent.
  9. `provider-brand-icons.ts` (`BRAND_ICON_KEYS`, `brand_icon_asset_filename()`, `brand_icon_asset_map()`) — 61 khóa biểu tượng thương hiệu chuẩn hóa theo đường dẫn asset tĩnh.
  10. `settings-events.ts` (`BACKEND_SETTINGS_CHANGED_EVENT`) — hằng tên sự kiện đồng bộ cài đặt backend.
  11. `lib/utils.ts` (`cn_classes()`) — gộp và khử trùng lặp class token thuần logic; giải quyết xung đột utility CSS runtime do `tailwind-merge` đảm nhiệm ở lớp React/CSS.
  12. `legacy-webkit.ts` (`resolve_relative_index()`) — ngữ nghĩa chỉ số âm tương đương `Array.prototype.at` / `TypedArray.prototype.at`.
- **`js_only` helpers trên ranh giới JS <-> WASM**:
  - Một số hàm nhận/trả `Set`, `Map`, callback, hoặc đọc `Date.now()` / `localStorage` / `navigator` (`unlistedModels`, `defaultPicked`, `busyLinks`, `returningLinks`, `routeRecordsByEntry`, `hubProviders`, `createRelayApiKey`, `createRelayProfile`, `createAggregateRelayProfile`, `duplicateRelayProfile`, `UpdateNotifications`, `ProviderBalanceStore`, `OpenPopoverRegistry`, `defaultDreamSkinTheme`, v.v.) giữ wrapper JS tương thích ở crate `rust-anything` nhưng đã có bản Rust thuần xác định tại [`manager-core-rs::js_only_bridges`](../crates/manager-models/manager-core-rs/src/js_only_bridges.rs).

## 2. Ranh giới giữ lại phía JS/Vite/WebView (`3 blocked` + `75 keep`)

- **`3` unit `blocked` (kịch bản kiểm thử thủ công DOM/Playwright/CJS)**:
  - `src/model-picker-views-runtime.manual.ts`
  - `src/runtime-context-capnweb-failure.manual.cjs`
  - `src/skills-batch-react.manual.ts`
  - Lý do: chạy kiểm thử tích hợp DOM/dynamic import trực tiếp trong JS/Node/browser, không thuộc logic production của Manager.
- **`75` unit `keep` (lớp UI React/Tailwind/Tauri shell cho Giai đoạn 5)**:
  - `65` component/trang `.tsx` đơn lẻ (bao gồm `src/App.tsx`, các trang/dialog/panel).
  - `src/components/` (`ProviderPresetSelector.tsx`) và `src/components/ui/` (`badge.tsx`, `button.tsx`, `card.tsx`, `input.tsx`, `label.tsx`, `textarea.tsx`).
  - `web-preview/` (`FlowDemo.tsx`, `entry.tsx`, và các Tauri web shims).
  - `7` module `.ts` gắn chặt React hooks / Tauri runtime store: `ReleaseNotes.ts`, `agent-focus-store.ts`, `command-palette-model.ts`, `gateway-store.ts`, `use-escape-layer.ts`, `use-modal-focus.ts`, `use-popover-layer.ts`.

## 3. Các điểm chạm runtime cụ thể cần giữ ở shell JS/Tauri

1. **Asset URL của Vite (`import.meta.url`)**: `provider-brand-icons.ts` trả URL đóng gói runtime bởi Vite bundler (`new URL("./assets/providers/<key>.svg", import.meta.url).href`). Rust giữ bảng 61 key và tên file tương đối `assets/providers/<key>.svg`.
2. **Polyfill WKWebView cũ (`legacy-webkit.ts`)**: Can thiệp prototype `Array.prototype.at`, `String.prototype.at`, `TypedArray.prototype.at`, `Object.hasOwn`, `structuredClone`, `crypto.randomUUID`, `IntersectionObserver` trong Safari/WKWebView macOS cũ — bắt buộc giữ dưới dạng JS early-init side effect.
3. **Browser globals & React subscription**: `window.dispatchEvent(new CustomEvent(...))`, `localStorage`, `window.matchMedia("(prefers-color-scheme: dark)")`, và `document.documentElement.classList` là hiệu ứng DOM thuần trình duyệt; Rust chỉ xử lý state chuyển đổi thuần túy và trả diff/state mới.
4. **`42-provider-editing.generated.js` (`assets/inject/renderer-inject/`)**: Bundle IIFE sinh bởi [`scripts/assemble-provider-editing.mjs`](../../scripts/assemble-provider-editing.mjs) cho Codex Desktop WebView (`window.codexPlusProviderEditing`). Parity 1:1 giữa bundle này với [`manager-core-rs::relay_editing`](../crates/manager-models/manager-core-rs/src/relay_editing.rs) đã được khóa bằng integration test trong [`manager_core_tests.rs`](../crates/manager-models/manager-core-rs/tests/manager_core_tests.rs).
