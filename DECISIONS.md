# Architectural Decision Records (`DECISIONS.md`) — `CodexPlusPlus` Rust Port

Tài liệu này lưu trữ các quyết định thiết kế cho quá trình port `CodexPlusPlus` sang Rust theo quy trình [`whole-app.md`](./skills/rust-anything/whole-app.md). Các quyết định nền tảng đã được người dùng phê duyệt ngày 2026-11-24.

---

## ADR-001: Cô lập không gian làm việc bản port trong `rust-port/`
- **Status**: `accepted` (2026-11-24)
- **Context**: Kho mã nguồn hiện tại đang ở trạng thái sạch tại tag `v2.8.0` và quy tắc dự án yêu cầu không sửa đổi tùy tiện `Cargo.toml`, `package.json`, `.gitignore` gốc hay làm hỏng mã đối chiếu trong quá trình `record` / `diff`.
- **Decision**: Đặt toàn bộ cấu hình MCP, skill, báo cáo phân tích, tài liệu quản trị (`MAP.md`, `DECISIONS.md`, `GLOSSARY.md`) và các crate mới của bản port bên trong `d:\Git\codex-plus\rust-port\`. Tái sử dụng trực tiếp `crates/codex-plus-core`, `crates/codex-plus-data` và `apps/codex-plus-launcher` qua path dependency khi cần tích hợp.
- **Consequences**: Mã nguồn gốc luôn nguyên vẹn 100% để chạy `rust-anything record`, `diff` và `app replay` bất cứ lúc nào.

## ADR-002: Kiến trúc đích cho tầng giao diện `codex-plus-manager` (75 UI `.tsx` units vs 83 Logic `.ts` units)
- **Status**: `accepted` (2026-11-24)
- **Context**: Backend của `CodexPlusPlus` đã viết bằng Rust (~316K dòng), nhưng `apps/codex-plus-manager/src` có 55.773 dòng TS/TSX gồm **83 units logic/model thuần** (`76 ready`, `7 caution`) và **75 units giao diện React `.tsx`** (`keep`).
- **Decision**: Thực hiện **Phương án A (Dioxus / Leptos Rust UI trên Tauri v2 tái sử dụng 30K dòng CSS gốc ở Giai đoạn 5)** sau khi port và kiểm chứng trước toàn bộ **83 units logic/model thuần** sang Rust/WASM crates thông qua `wasm-bindgen` + 236 file test sẵn có.
- **Consequences**: Toàn bộ ứng dụng Manager đạt 100% Rust mà không đánh mất giao diện và CSS hiện có; mọi crate `port.rs` đều biên dịch được thành cả `rlib` (cho Rust GUI/Tauri host) lẫn `wasm32-unknown-unknown` (cho WebView).

## ADR-003: Cơ chế Bridge trên Windows (`wasm-bindgen` vs `napi-rs`)
- **Status**: `accepted` (2026-11-24)
- **Context**: Máy phát triển Windows hiện dùng `stable-x86_64-pc-windows-gnu` (WinLibs GCC) không cài Visual Studio MSVC `cl.exe`/`link.exe`, đồng thời các module frontend & `conversation-canvas` chạy trong môi trường trình duyệt/WebView.
- **Decision**: Ưu tiên `--kind wasm` (`wasm32-unknown-unknown` + `wasm-bindgen 0.2.130`, đã kiểm chứng chạy E2E `record -> build -> diff -> gate` thành công trên máy) cho toàn bộ TS/JS library & model units; sử dụng `--allocator rusty_alloc` khi bật `--batch` để không phụ thuộc trình biên dịch C MSVC. Với các CLI scripts (`scripts/`, `tools/codex-wechat`), dùng luồng `app record` -> `app front-door` (`clap`) -> `app lift` ra binary `.exe` thuần Rust trên `x86_64-pc-windows-gnu`. Mỗi unit crate trong `rust-port/crates/` khai báo bảng `[workspace]` rỗng để cô lập khỏi `Cargo.toml` gốc.
- **Consequences**: Mọi bước `record`, `build`, `diff`, `gate` chạy mượt 100% ngay trên môi trường Windows hiện tại mà không cần cài thêm Visual Studio Build Tools nhiều GB.

## ADR-004: Hợp nhất `scripts/` (29 Node/Python scripts) thành CLI thuần Rust `codex-plus-devkit`
- **Status**: `accepted` (2026-11-24)
- **Context**: Thư mục `scripts/` gồm 28 file `.mjs`/`.cjs` và 1 file `.py` (build, assemble inject shards, phát hành `release.mjs`, khởi chạy cục bộ `relaunch-local.mjs` hiện chỉ chạy được trên macOS).
- **Decision**: Port các script sản xuất/build/release thành một binary CLI thuần Rust đa nền tảng `codex-plus-devkit` (hỗ trợ cả Windows, macOS, Linux); giữ hoặc thay các bài test Playwright CDP bằng `chromiumoxide` / Rust E2E ở giai đoạn cuối.
- **Consequences**: Xóa bỏ phụ thuộc Node/Python cho quy trình build/release và hỗ trợ `relaunch-local` trực tiếp trên Windows.

## ADR-005: Thay thế Babel AST Inspector (`assets/native-browser`) bằng `oxc_parser` thuần Rust
- **Status**: `accepted` (2026-11-24)
- **Context**: `native_browser.rs` hiện phải gọi tiến trình `node` ngoài để chạy `inspect-service.cjs` (bundle `@babel/parser` + `@babel/traverse` dài 394 dòng) mỗi khi kiểm tra bản vá trình duyệt.
- **Decision**: Port `inspect-service.mjs` và `require-identification.mjs` sang crate Rust dùng `oxc_parser` + `oxc_ast_visit`, kiểm chứng khớp 100% JSON schema đầu ra bằng `rust-anything diff`, sau đó nhúng thẳng vào Rust (không cần gọi `node` subprocess).
- **Consequences**: Tăng tốc độ khởi chạy, loại bỏ rủi ro thiếu `node` trên máy người dùng cuối.

## ADR-006: Ranh giới cho `assets/inject/renderer-inject` & `assets/claude-localization`
- **Status**: `accepted` (2026-11-24)
- **Context**: Codex Desktop và Claude Desktop đều là ứng dụng Electron bên thứ ba chạy V8 trong tiến trình của chúng.
- **Decision**:
  - Với `assets/claude-localization`: Port toàn bộ công cụ build/ký/tiêm bên ngoài (`build-copy.py`, `sign-copy.py`, `build-copy.ps1`, `inject-cn.ps1`, `inject-cn.js`) sang Rust thuần. Giữ các tệp chạy *bên trong* Electron (`hook.js`, `page-shim.js`, `page-dictation.js`, `bash-timeout.js`) làm runtime shim tối giản (kèm vá lỗi quyền `defaultSession` từ báo cáo review).
  - Với `assets/inject/renderer-inject`: Giữ lớp gắn kết DOM/V8 ở dạng JS mỏng (`keep`), chuyển phần xử lý dữ liệu nặng (`42-provider-editing`, `70-model-catalog`, `96-prompt-args`, `34-statsig-overrides`) sang dùng chung crate WASM hoặc gọi Rust helper API.
- **Consequences**: Đạt mức Rust tối đa về mặt kỹ thuật mà vẫn tương thích 100% với Electron của OpenAI và Anthropic.

## ADR-007: Thanh chuẩn tương đương hành vi (Equivalence Bar) & Sửa lỗi bảo mật/logic
- **Status**: `accepted` (2026-11-24)
- **Context**: Trong đợt code review trước đã phát hiện một số lỗi trong mã gốc (như rò rỉ khóa ở `/settings/get`, `POST /api/shares` mở không xác thực trên `share-site`, `sign-copy.py` thoát lỗi sai khi tự ký).
- **Decision**: Theo quy tắc của `rust-anything`, bước port đầu tiên tuân thủ **khớp hành vi 100% (`diff` N/N)** với bản gốc; mọi thay đổi cố ý nhằm vá lỗi bảo mật/logic của bản gốc đều được ghi rõ từng dòng vào `<crate>/known-diffs.jsonl` kèm lý do sau khi người dùng đồng ý.
- **Consequences**: Không bao giờ trộn lẫn lỗi dịch mã (port bug) với bản vá chủ đích.
