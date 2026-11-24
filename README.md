# CodexPlusPlus — Pure-Rust Standalone Edition

Repository mã nguồn thuần Rust độc lập của **CodexPlusPlus** (`https://github.com/OrgGem/codex-plus-rust`) — không phụ thuộc Node.js, npm, Vite, Electron, hay Python ở bước build và runtime.

> Bạn có thể làm việc trực tiếp ngay trong thư mục này (hoặc clone độc lập repo `OrgGem/codex-plus-rust`) để build, chạy test, đóng gói bản phát hành portable và phát triển tính năng mới mà không cần kho gốc.

---

## Kiến trúc tổng quan

```text
codex-plus-rust/
├── Cargo.toml                         # Root Cargo workspace hợp nhất (16 crate thành viên)
├── build-release.ps1 / .sh            # Script build, test, smoke-check & đóng gói portable 1 lệnh
├── cutover.ps1 / .sh                  # Kiểm tra preflight & chuyển đổi / rollback an toàn
├── AGENTS.md                          # Quy ước phát triển, build, test & bảo mật cho repo độc lập
├── CHANGELOG.md                       # Lịch sử phiên bản (dùng bởi codex-plus-devkit release)
├── EXTENSIONS.md                      # Đặc tả API mở rộng `window.codexPlus`
├── THIRD_PARTY_NOTICES.md             # Thông báo bản quyền thư viện bên thứ ba
├── apps/
│   ├── codex-plus-launcher/           # Core launcher & daemon gateway helper
│   └── codex-plus-manager/src-tauri/  # Biểu tượng & Windows manifest cho launcher
├── assets/
│   ├── manager-css/                   # 27 file CSS gốc (nhúng vào binary qua build.rs)
│   ├── inject/                        # Renderer-inject shards & extension assets
│   ├── claude-localization/           # Gói bản địa hóa Claude Desktop
│   ├── native-browser/                # Fixture kiểm thử hợp đồng trình duyệt nội bộ
│   ├── plugin-marketplaces/           # Snapshot marketplace tích hợp
│   └── ...                            # Preset MCP, model catalog mặc định, icon, i18n
├── crates/
│   ├── codex-plus-app/                # Ứng dụng Desktop GUI thuần Rust (Dioxus 0.7 + WebView2)
│   │   └── sherpa-onnx-gnu-shim/      # Shim MinGW GNU an toàn cho sherpa-onnx trên Windows
│   ├── codex-plus-core/               # Core logic: Relay profile, model_catalog_json, Gateway, ACL, MCP
│   ├── codex-plus-data/               # Persistence SQLite WAL, usage archive, session history
│   ├── codex-plus-devkit/             # CLI DevOps: release-meta, assemble-renderer-inject, relaunch-local, native-browser-inspector
│   ├── manager-models/
│   │   ├── manager-common-rs/         # Helper chung cho mô hình quản lý UI
│   │   ├── manager-core-rs/           # Logic UI/state thuần Rust + WASM
│   │   └── *-rs/                      # Các unit crate độc lập (rust-anything gate)
│   ├── renderer-inject-rs/            # Core Rust/WASM cho prompt-args, statsig, model-catalog, XSS sanitizer
│   ├── claude-patcher-rs/             # CLI patch ASAR, Fuses, Entitlements & CDP cho Claude Desktop
│   ├── codex-wechat-rs/               # Cầu nối WeChat iLink <-> Codex thuần Rust
│   ├── remote-control-rs/             # CLI điều khiển từ xa (remote-control-p0 & IPC)
│   ├── canvas-wasm/                   # Core Rust/WASM cho Conversation Canvas
│   ├── share-site-rs/                 # Mã hóa AES-256-GCM & xuất Markdown chia sẻ hội thoại
│   ├── rice-maid-pet-rs/              # Backend & layout pet đồng hành
│   └── native-browser-inspector-rs/   # Trình phân tích AST oxc_parser thay thế Babel/Node
└── docs/
    ├── FEATURES.md                    # Bảng đặc tả chi tiết toàn bộ chức năng ứng dụng
    ├── images/                        # Tài nguyên hình ảnh nhúng vào core
    └── *.md                           # 27 tài liệu thiết kế chuyên sâu từng phân hệ
```

---

## Yêu cầu môi trường

- **Rust toolchain**: `rustc >= 1.97` (edition `2024`).
  - Trên Windows (MinGW GNU): `rustup target add x86_64-pc-windows-gnu`
  - Nếu biên dịch các crate WASM: `rustup target add wasm32-unknown-unknown`
- **Windows Runtime**: Microsoft Edge WebView2 (có sẵn trên Windows 10/11). `build.rs` tự động copy `WebView2Loader.dll` cạnh file thực thi.
- **Không cần**: Node.js, npm, Vite, Python hay Tauri CLI.

---

## Hướng dẫn nhanh (Build, Test, Chạy & Đóng gói)

Tất cả các lệnh bên dưới chạy trực tiếp tại thư mục gốc của repo này:

### 1. Chạy toàn bộ bộ test workspace

```powershell
cargo test
```

Kiểm tra các crate lõi (`codex-plus-core`, `codex-plus-data`):

```powershell
cargo test -p codex-plus-core --lib
cargo test -p codex-plus-data --lib
```

Kiểm tra biên dịch WASM:

```powershell
cargo check --target wasm32-unknown-unknown -p manager-core-rs -p renderer-inject-rs -p canvas-core-rs -p share-site-rs -p rice-maid-pet-rs
```

### 2. Chạy ứng dụng Desktop khi đang phát triển

```powershell
cargo run -p codex-plus-app
```

Chạy smoke-check không mở cửa sổ (xác minh Local Gateway HTTP 200, tải preset và phiên làm việc):

```powershell
cargo run -p codex-plus-app -- --smoke-check target/smoke.json
```

### 3. Build & đóng gói bản phát hành Release (1 lệnh)

**Windows (PowerShell):**

```powershell
powershell -ExecutionPolicy Bypass -File .\build-release.ps1
```

Các cờ tùy chọn:
- `-SkipTests`: bỏ qua bước `cargo test` để build nhanh.
- `-Offline`: chạy Cargo với `--offline`.
- `-SkipZip`: chỉ tạo thư mục portable, không nén `.zip`.
- `-Launch`: tự động mở `codex-plus-app.exe` sau khi build & smoke-check thành công.

Sản phẩm xuất ra tại `release-bundle/CodexPlusPlus-Rust-v<version>-win-x64/` kèm `SHA256SUMS.txt`, `Start-CodexPlusPlus-Portable.cmd` và file `.zip`.

**macOS / Linux:**

```bash
./build-release.sh
```

### 4. Kiểm tra Preflight / Cutover

```powershell
powershell -ExecutionPolicy Bypass -File .\cutover.ps1 -Check
```

---

## Các Binary chính được sinh ra

| Binary | Crate | Chức năng |
| --- | --- | --- |
| `codex-plus-app` | `crates/codex-plus-app` | Ứng dụng Desktop Manager (Dioxus 0.7) + Local Gateway nội bộ |
| `codex-plus-devkit` | `crates/codex-plus-devkit` | CLI DevOps (`release-meta`, `release`, `assemble-renderer-inject`, `relaunch-local`, `prune-local-builds`, `build-plugin-market-index`, `sync-official-models`, `inspect-browser-service`) |
| `native-browser-inspector` | `crates/codex-plus-devkit` | Kiểm tra hợp đồng AST dịch vụ trình duyệt bằng `oxc_parser` |
| `codex-wechat` | `crates/codex-wechat-rs` | Cầu nối nhắn tin WeChat iLink <-> Codex |
| `claude-patcher` | `crates/claude-patcher-rs` | Bản địa hóa & patch ASAR/Fuses/CDP cho Claude Desktop |
| `remote-control-p0` | `crates/remote-control-rs/remote-control-p0` | Relay & CLI điều khiển phiên làm việc từ xa |

---

## Tài liệu tham khảo

- **Đặc tả đầy đủ tính năng**: [docs/FEATURES.md](docs/FEATURES.md)
- **Hướng dẫn phát triển độc lập & quy tắc an toàn**: [AGENTS.md](AGENTS.md)
- **API mở rộng `window.codexPlus`**: [EXTENSIONS.md](EXTENSIONS.md)
- **Lịch sử thay đổi**: [CHANGELOG.md](CHANGELOG.md)
- **Tài liệu chuyên sâu từng phân hệ**: xem toàn bộ 27 tài liệu trong thư mục `docs/`.
