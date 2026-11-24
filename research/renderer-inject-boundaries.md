# `assets/inject/renderer-inject` Runtime Boundaries (`ADR-006`)

Phân hệ `assets/inject/renderer-inject` gồm `56` phân mảnh (~`38.943` dòng JS chạy bên trong tiến trình Electron renderer của Codex Desktop). Trong [`DECISIONS.md`](../DECISIONS.md) (`ADR-006`) và [`analysis/assets-renderer-inject.md`](../analysis/assets-renderer-inject.md):

- **`blocked (3)` — giữ ở runtime JS/Electron**:
  - `20-menu.js`, `91-extension-runtime.js`, `96-dsh-client.js`: sử dụng `new Function` / dynamic runtime code evaluation hoặc Nạp script người dùng trong browser context.
- **Lớp gắn DOM, CDP Bridge & React Internals — giữ shim mỏng ở JS**:
  - Các hook lắng nghe sự kiện `document` / `window` / `MutationObserver` / `__reactFiber$` / TanStack `QueryClient` / CSS Variables (`00-prelude.js`, `01-registry.js`, `10-style.js`, `50-navigation.js`, `60..62`, `80..86`, `92..95`, `98..99`).
  - Bảo đảm tuân thủ chuẩn: giữ nguyên tên `window.codexPlus.constants` và đánh dấu node chèn bằng `[data-codex-plus-ext]`.
- **Phần tính toán thuần túy & sanitizer bảo mật — đã port sang [`renderer-inject-rs`](../crates/renderer-inject-rs/Cargo.toml)** (cùng `manager-core-rs`):
  1. `42-provider-editing.generated.js`: Logic `80` hàm cấu hình provider đã được phủ 100% qua `manager-models/relay-profile-editing-rs` và `manager-core-rs::relay_editing` (đã khóa differential test với bundle JS qua Node `vm`).
  2. `96-prompt-args.js`: `prompt_args_parse_command`, `prompt_args_split`, `prompt_args_placeholders`, `prompt_args_substitute`, `prompt_args_expand`.
  3. `34-statsig-overrides.js`: `statsig_gate_catalog`, `statsig_user_gate_overrides`, `statsig_effective_gates`, `statsig_merge_object_value`, `statsig_overrides_signature`, `StatsigGateRecorder` (`limit = 500`).
  4. `70-model-catalog.js` & `71-model-picker-groups.js`: `unique_model_values`, `model_settings_source`, `model_settings_key`, `model_catalog_identity`, `catalog_belongs_to_profile`, `effective_model_names`, `lookup_model_metadata`, `model_reasoning_efforts`, `build_model_descriptor`, `apply_model_metadata_to_descriptor`, `model_response_host_id`, `patch_model_defaults_pure`, `parse_picker_group_suffix`.
  5. `90-action-groups.js`, `91-extension-api.js`, `92-extension-host.js`, `40-backend-settings.js`: `EXTENSION_ALLOWED_ROUTES` (`25` routes an toàn, chặn `/hooks/test-run`, `/hooks/file/write`, `/settings/set`, `/delete` theo `C-3`), `extension_coalesce_window_ms`, và các hàm chống DOM XSS `H-5` (`escape_html`, `escape_css_ident`, `sanitize_svg_icon`, `sanitize_external_http_url`) ghi nhận tại [`known-diffs.jsonl`](../crates/renderer-inject-rs/known-diffs.jsonl).
