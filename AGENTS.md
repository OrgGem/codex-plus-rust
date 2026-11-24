# AGENTS.md

Hướng dẫn làm việc dành cho developer và AI agent khi phát triển độc lập trong repository **CodexPlusPlus Pure-Rust** (`https://github.com/OrgGem/codex-plus-rust`).

## Tổng quan dự án

Repository này là bản mã nguồn thuần Rust độc lập hoàn toàn của **CodexPlusPlus** (không yêu cầu Node.js, npm, Vite hay Python):

- `Cargo.toml` — Root Cargo workspace hợp nhất cho toàn bộ 16 crate sản phẩm.
- `crates/codex-plus-app/` — Ứng dụng Desktop GUI thuần Rust (Dioxus 0.7 + WebView2).
- `crates/codex-plus-core/` — Lõi nghiệp vụ (Relay profile, `model_catalog_json`, Gateway, MCP, ACL, Claude local).
- `crates/codex-plus-data/` — Persistence SQLite WAL, lịch sử phiên và usage archive.
- `apps/codex-plus-launcher/` — Core launcher & helper gateway.
- `crates/codex-plus-devkit/` — Bộ công cụ DevOps thuần Rust (`codex-plus-devkit`, `native-browser-inspector`).
- `crates/manager-models/`, `crates/renderer-inject-rs/`, `crates/canvas-wasm/` — Các thư viện logic dùng chung (hỗ trợ biên dịch native & WASM).
- `crates/claude-patcher-rs/`, `crates/codex-wechat-rs/`, `crates/remote-control-rs/`, `crates/share-site-rs/`, `crates/rice-maid-pet-rs/` — Các công cụ & dịch vụ vệ tinh thuần Rust.
- `assets/` — CSS gốc (`assets/manager-css/`), shard `renderer-inject`, preset, bản địa hóa, marketplace zip nhúng sẵn.
- `docs/` — Đặc tả chức năng ([docs/FEATURES.md](docs/FEATURES.md)) và 27 tài liệu thiết kế từng phân hệ.

## Quy tắc an toàn

- **Cấm xóa hàng loạt**: Không dùng `rm -rf`, `rmdir /s`, `Remove-Item -Recurse`. Chỉ xóa từng tệp khi thực sự cần và đã xác nhận.
- **Cấm leo thang đặc quyền / tải script thực thi**: Không dùng `sudo`, `curl | bash`.
- **Bảo vệ bí mật**: Không đọc/in/commit `.env`, `auth.json`, `settings.json`, `state/`, khóa API hay thông tin đăng nhập trong `config.toml`.
- **Không commit sản phẩm build**: `target/`, `release-bundle/`, `*.exe`, `*.dll`, `*.zip` đều bị `.gitignore` loại trừ — **ngoại lệ duy nhất bắt buộc giữ trong Git** là `assets/plugin-marketplaces/*.zip` vì `crates/codex-plus-core/src/plugin_marketplace.rs` nhúng trực tiếp bằng `include_bytes!`.

## Quy ước phát triển & kiểm thử

- Chạy test workspace mặc định:
  ```powershell
  cargo test
  ```
- Kiểm tra biên dịch WASM:
  ```powershell
  cargo check --target wasm32-unknown-unknown -p manager-core-rs -p renderer-inject-rs -p canvas-core-rs -p share-site-rs -p rice-maid-pet-rs
  ```
- Sửa các phân mảnh `assets/inject/renderer-inject/*.js` xong phải kiểm tra / đóng gói lại bằng:
  ```powershell
  cargo run -p codex-plus-devkit -- assemble-renderer-inject --root .
  ```
- Sửa giao diện CSS đặt trong `assets/manager-css/` (được `crates/codex-plus-app/build.rs` nhúng tự động lúc build).
- Giữ nguyên hợp đồng `window.codexPlus` (không đổi tên class/constant đã công bố, không mở rộng route ghi vào whitelist extension).
- Mọi tính năng mới hoặc sửa lỗi phải đi kèm unit/integration test (`#[test]` + `tempfile`).

## Quy trình Build & Xác minh trước khi bàn giao

Sau khi hoàn thành sửa đổi và test đạt 100%:

1. Build & đóng gói bản release + chạy `--smoke-check` tự động:
   - Windows:
     ```powershell
     powershell -ExecutionPolicy Bypass -File .\build-release.ps1 -Launch
     ```
   - macOS / Linux:
     ```bash
     ./build-release.sh
     ```
2. Kiểm tra preflight toàn diện:
   ```powershell
   powershell -ExecutionPolicy Bypass -File .\cutover.ps1 -Check
   ```
3. Kiểm tra phiên bản đồng bộ trước khi phát hành:
   ```powershell
   cargo run -p codex-plus-devkit -- release-meta check <version> --root .
   ```

## Quy ước Git & Phát hành

- Không thêm dòng `Co-Authored-By:` của AI vào commit message hay PR description.
- Trước khi nâng phiên bản: cập nhật mục `## X.Y.Z - YYYY-MM-DD` đầu file `CHANGELOG.md`, sau đó chạy `codex-plus-devkit release X.Y.Z --dry-run --root .` để xem trước trước khi phát hành.
