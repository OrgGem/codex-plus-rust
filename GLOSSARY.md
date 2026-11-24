# Glossary — Ánh Xạ Thuật Ngữ & Kiểu Dữ Liệu (`CodexPlusPlus` -> Rust)

Tài liệu này chuẩn hóa tên gọi kiểu dữ liệu, module và quy ước chuyển đổi ngữ nghĩa giữa TypeScript/JavaScript/Python/PowerShell và các crate Rust trong `rust-port/crates/`, đảm bảo tính nhất quán xuyên suốt các Wave.

| Thuật ngữ mã gốc | Ngữ cảnh / Nguồn | Tên & Kiểu chuẩn trong Rust | Quy ước ngữ nghĩa cần giữ |
| :--- | :--- | :--- | :--- |
| `RelayProfile` | `src/relay-profile-editing.ts`, `codex-plus-core::settings` | `RelayProfile` (`serde` camelCase) | Giữ nguyên thứ tự khóa mở rộng (`IndexMap` / `serde_json::Map` với `preserve_order`). |
| `GatewayPayload` / `ProviderPool` | `src/gateway-model.ts`, `gateway_commands.rs` | `GatewaySnapshot` / `ProviderEntry` | Chuỗi định danh `<provider>/<model>` và `group/<id>` phân tách bằng `split_once('/')`. |
| `ModelWindow` (`1M`, `200K`) | `src/model-windows.ts`, `model_catalog.rs` | `ContextWindowSpec` (`u64`) | Parse hậu tố `K`/`M` không phân biệt hoa thường, kiểm tra tràn số `i64::try_from`. |
| `SkillGroup` / `SkillBatch` | `src/skill-groups.ts`, `src/skill-agent-batch.ts` | `SkillGroup` / `SkillBatchPlan` | Bảo toàn thứ tự chèn nhóm (`IndexMap<String, SkillGroup>`). |
| `HighlightToken` | `src/code-highlight.ts` | `HighlightSpan { kind: TokenKind, text: String }` | Nối chuỗi toàn bộ `text` phải khớp byte-for-byte với đầu vào; chú ý chỉ số UTF-16 (`src/utf16.rs`) nếu trả offset cho JS. |
| `SettingsRebase` | `src/settings-rebase.ts` | `rebase_settings_edits` | Merge ba chiều (`base`, `draft`, `remote`) bảo toàn OAuth token mới xoay vòng. |
| `CanvasTree` / `TurnNode` | `tools/conversation-canvas/model.mjs` | `CanvasTree` / `TurnNode` | ID nút chuỗi, thứ tự con giữ nguyên thứ tự xuất hiện trong lịch sử hội thoại. |
| `BrowserServiceInspection` | `assets/native-browser/inspect-service.mjs` | `ServiceAstReport` (dùng `oxc_parser`) | Khớp chính xác JSON schema đầu ra mà `native_browser.rs` đang đọc. |
| JS `Map` / Object key order | Toàn bộ TS/JS units | `indexmap::IndexMap` | Dùng `shift_remove` thay vì `swap_remove` để khớp hành vi `delete obj[k]` của V8. |
| JS Regex `/.../i` (không cờ `u`) | `src/*.ts`, `scripts/*.mjs` | `regex` với `(?i-u)` hoặc `fancy_regex` | Tránh khớp ASCII với ký tự Unicode đặc biệt (`ſ` vs `s`, `K` vs `k`). |
