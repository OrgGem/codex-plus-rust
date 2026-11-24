# Đặc tả Chức năng — CodexPlusPlus (Pure-Rust Edition)

Tài liệu này mô tả đầy đủ các nhóm chức năng, kiến trúc mô-đun, điểm vào mã nguồn và hợp đồng an toàn của **CodexPlusPlus Pure-Rust Edition** để phục vụ việc phát triển độc lập trong repository này.

---

## 1. Quản lý nhà cung cấp (Relay Providers) & Cửa sổ ngữ cảnh theo từng Model

- **Mục đích**: Cho phép cấu hình nhiều nhà cung cấp API (OpenAI-compatible, Anthropic/Claude, relay nội bộ, v.v.), nhiều API key đặt tên (`named API keys`), kiểm tra kết nối, theo dõi số dư và chuyển đổi nhanh profile mà không sửa tay `~/.codex/config.toml`.
- **Cửa sổ ngữ cảnh theo model (`model_list` + `model_catalog_json`)**:
  - Hỗ trợ cú pháp hậu tố trực tiếp trên danh sách model, ví dụ `deepseek-v4-pro[1M]` hoặc `claude-sonnet-4[200k]`.
  - Tự động sinh file `model_catalog_json`, tiêm đường dẫn vào `config.toml` và thiết lập ngưỡng tự động nén ngữ cảnh (`auto_compact_limit`) riêng cho từng model mà không phá vỡ cấu hình mặc định của profile.
- **Mã nguồn chính**:
  - `crates/codex-plus-core/src/settings.rs` (`RelayProfile`, cài đặt toàn cục)
  - `crates/codex-plus-core/src/relay_config.rs` (`apply_context_limits_to_config`, `apply_relay_profile_to_home_with_switch_rules`)
  - `crates/codex-plus-core/src/model_catalog.rs` (`parse_model_catalog_json_models`)
  - `crates/manager-models/manager-core-rs/` (logic form provider, nhóm model, routing, cân bằng tải)
- **Tài liệu chi tiết**: [model-window-overrides.md](model-window-overrides.md), [named-api-keys.md](named-api-keys.md), [local-config-provider.md](local-config-provider.md).

---

## 2. Local Gateway & Proxy giao thức nội bộ

- **Mục đích**: Đóng vai trò cổng trung chuyển cục bộ giữa Codex Client / Claude Desktop và các nhà cung cấp upstream, hỗ trợ chuyển đổi giao thức, ghi log request, đếm token/chi phí thực tế, bộ nhớ đệm nén cục bộ (`local compaction cache`), chuyển mạch dự phòng và bảo vệ đa phiên.
- **Điểm đầu cuối nội bộ chính**:
  - Cổng gateway cục bộ (mặc định loopback `127.0.0.1:57321`, cấu hình được).
  - `/backend/status` và `/proxy-status`: báo cáo sức khỏe dịch vụ, cấu hình đang chạy, trạng thái nhà cung cấp và danh sách route.
  - Theo dõi lịch sử request, thống kê chi phí/usage theo ngày/model/nhà cung cấp và lưu trữ dài hạn vào SQLite.
- **Mã nguồn chính**:
  - `apps/codex-plus-launcher/src/main.rs` (vòng đời daemon helper & single-instance guard)
  - `crates/codex-plus-core/src/` (gateway server, proxy, routing, auto-compact, usage metering)
  - `crates/codex-plus-data/src/` (SQLite WAL + `busy_timeout`, kho lưu trữ usage archive)
- **Tài liệu chi tiết**: [local-gateway.md](local-gateway.md), [local-compaction-cache.md](local-compaction-cache.md), [usage-history.md](usage-history.md), [usage-archive.md](usage-archive.md), [usage-supplier-evidence.md](usage-supplier-evidence.md).

---

## 3. Ứng dụng Desktop Manager thuần Rust (`codex-plus-app`)

- **Công nghệ**: Dioxus `0.7` Desktop (WebView2 trên Windows, WKWebView trên macOS, WebKitGTK trên Linux) — thay thế hoàn toàn Tauri + React + Vite + Node.js.
- **Năng lực giao diện**:
  - **32 màn hình / route quản trị**: Tổng quan (Overview / Hub), Nhà cung cấp (Providers), Gateway & Nhật ký Request, Phiên làm việc nội bộ (Sessions), Quản lý MCP, Khám phá Plugin & Skill Marketplace, Tiện ích mở rộng (Extensions & User-Scripts), Cấu hình nâng cao (Config TOML / Config Doctor), Khởi chạy đa phiên (Multi-Launch), Thống kê sử dụng (Usage Analytics), Chế độ Claude Desktop, Điều khiển từ xa (Remote Control), WeChat Bridge, và Cài đặt hệ thống.
  - **Nhúng sẵn 27 file CSS thiết kế gốc** (`assets/manager-css/*.css` được `crates/codex-plus-app/build.rs` hợp nhất và tự động chuyển `@theme {` sang `:root {` để giữ nguyên biến thiết kế Dark/Light).
  - **Command Palette (`Ctrl+K`)**: Tìm kiếm và chuyển trang nhanh không cần rời bàn phím.
  - **Tô sáng cú pháp TOML / JSON zero-copy** cho trình xem/sửa cấu hình và log.
  - **Chế độ kiểm thử không mở cửa sổ (`--smoke-check <path>`)**: Khởi động backend, gọi thật `/backend/status` (HTTP 200), đếm phiên làm việc & preset MCP, xuất báo cáo JSON và thoát sạch.
- **Mã nguồn chính**:
  - `crates/codex-plus-app/src/main.rs` (entry point, CLI `--smoke-check`, khởi chạy Dioxus Desktop)
  - `crates/codex-plus-app/src/backend.rs` (cầu nối bất đồng bộ tới `codex-plus-core` / `codex-plus-data` / Local Gateway)
  - `crates/codex-plus-app/src/pages/` & `components/` (các trang quản trị, layout, palette, theme)
  - `crates/codex-plus-app/build.rs` (đóng gói CSS & tự động stage `WebView2Loader.dll` trên Windows)
- **Tài liệu chi tiết**: [theme-shell-css.md](theme-shell-css.md), [isolated-launcher-testing.md](isolated-launcher-testing.md).

---

## 4. Quản lý phiên làm việc (Sessions), Khôi phục lượt hội thoại & Phân trang

- **Mục đích**: Liệt kê, phân trang, tìm kiếm, sửa chỉ mục bị hỏng (`session index repair`), quay lui lượt hội thoại (`turn rewind`), khôi phục lịch sử (`history turn recovery`) và xóa an toàn các phiên Codex cục bộ.
- **An toàn xóa tệp**: Mọi thao tác xóa phiên đều được khóa trong thư mục `sessions_root` sau khi chuẩn hóa canonical path (chống path traversal / symlink escape).
- **Mã nguồn chính**: `crates/codex-plus-core/src/`, `crates/codex-plus-data/src/`, `crates/codex-plus-app/src/backend.rs`.
- **Tài liệu chi tiết**: [session-paging.md](session-paging.md), [session-index-repair.md](session-index-repair.md), [turn-rewind.md](turn-rewind.md), [history-turn-recovery.md](history-turn-recovery.md), [managed-session-provider.md](managed-session-provider.md).

---

## 5. MCP Presets, Plugin Marketplace & Config Doctor

- **MCP Presets & Config Doctor**:
  - Cung cấp sẵn các preset MCP phổ biến (`assets/`), kiểm tra cấu hình `config.toml` theo bộ quy tắc Config Doctor và ghi file nguyên tử (`atomic write`).
  - Hỗ trợ kiểm soát quyền chạy lệnh, cấu hình subagent kế thừa model và đồng bộ mục tiêu cấu hình.
- **Plugin & Skill Marketplace**:
  - Hỗ trợ duyệt, cài đặt và lập chỉ mục plugin/skill từ gói tích hợp (`assets/plugin-marketplaces/openai-curated-remote.zip`) hoặc kho từ xa.
  - CLI `codex-plus-devkit build-plugin-market-index` sinh chỉ mục JSON chuẩn không cần Node.js.
- **Tài liệu chi tiết**: [plugin-market.md](plugin-market.md), [native-subagent-model-inheritance.md](native-subagent-model-inheritance.md), [agent-collaboration.md](agent-collaboration.md).

---

## 6. Hệ sinh thái Tiện ích mở rộng (Extensions) & `renderer-inject-rs`

- **Mục đích**: Cho phép tải user-scripts và extension chạy bên trong Codex Client thông qua API công khai `window.codexPlus`, đồng thời kiểm soát nghiêm ngặt danh sách route được phép gọi (whitelist 25 route chỉ đọc/an toàn, chặn tuyệt đối các route ghi nhạy cảm như `/settings/set`, `/delete`).
- **Lõi thuần Rust/WASM (`crates/renderer-inject-rs/`)**:
  - Xử lý tham số prompt (`prompt-args`), ghi đè cờ tính năng (`statsig-overrides`), quản lý danh mục model & nhóm bộ chọn (`model-catalog`, `picker-groups`).
  - Bộ lọc làm sạch SVG / HTML / URL chống XSS trước khi chèn vào DOM.
- **Đóng gói shard (`codex-plus-devkit assemble-renderer-inject`)**:
  - Hợp nhất các phân mảnh `assets/inject/renderer-inject/*.js` theo `manifest.json` và xác thực cú pháp JS trực tiếp bằng `oxc_parser` (không cần Node.js).
- **Tài liệu chi tiết**: [../EXTENSIONS.md](../EXTENSIONS.md), [user-script-reload.md](user-script-reload.md), [whale-widget.md](whale-widget.md), [whale-parity.md](whale-parity.md).

---

## 7. Trình kiểm tra AST trình duyệt nội bộ (`native-browser-inspector`)

- **Mục đích**: Thay thế kịch bản Node.js + Babel (`inspect-service.cjs`) bằng trình phân tích AST thuần Rust dựa trên `oxc_parser`.
- **Chức năng**: Xác minh hợp đồng ngữ nghĩa của dịch vụ native browser, kiểm tra điều kiện nhận diện tiện ích mở rộng Chrome/Edge (`eval_require_identification`) và ẩn toàn bộ đường dẫn nhạy cảm trong thông báo lỗi (exit code `2` với thông điệp cố định).
- **Mã nguồn chính**: `crates/native-browser-inspector-rs/`, `crates/codex-plus-devkit/src/browser_inspector.rs`.
- **Tài liệu chi tiết**: [native-browser-identification.md](native-browser-identification.md).

---

## 8. Bản địa hóa & Chế độ đầy đủ cho Claude Desktop (`claude-patcher-rs`)

- **Mục đích**: Hỗ trợ bản địa hóa giao diện Claude Desktop (tạo bản sao, giải nén/đóng gói ASAR, xử lý Electron Fuses, Entitlements, ký lại và tiêm gói ngôn ngữ qua CDP) cũng như chạy chế độ Claude Desktop tương thích qua `claude-local`.
- **Mã nguồn chính**:
  - `crates/claude-patcher-rs/` (CLI `claude-patcher`: `build-copy`, `sign-copy`, `inject-cn`)
  - `crates/codex-plus-core/src/claude_local_backend.rs` & `claude_localization.rs`
  - `assets/claude-localization/`
- **Tài liệu chi tiết**: [claude-desktop-extensions.md](claude-desktop-extensions.md).

---

## 9. Các công cụ & Thư viện vệ tinh thuần Rust

| Thành phần | Crate | Mô tả chức năng | Tài liệu liên quan |
| --- | --- | --- | --- |
| **WeChat Bridge** | `crates/codex-wechat-rs` (`codex-wechat`) | Kết nối tin nhắn 2 chiều giữa WeChat iLink và Codex `app-server`/`exec`, hỗ trợ mã hóa payload và lọc nguồn | [weixin-inbound.md](weixin-inbound.md), [weixin-outbound.md](weixin-outbound.md) |
| **Remote Control** | `crates/remote-control-rs/remote-control-p0` (`remote-control-p0`) | Trích xuất phiên bản ASAR, định tuyến IPC, `ThreadFollower`, sidecar app-server và relay điều khiển từ xa | — |
| **Conversation Canvas** | `crates/canvas-wasm/canvas-core-rs` | Lõi Rust/WASM tổ chức cây hội thoại, đồng bộ SSE, phân trang lịch sử và xuất userscript | — |
| **Share Site** | `crates/share-site-rs` | Router chia sẻ hội thoại, mã hóa đầu cuối AES-256-GCM và xuất Markdown an toàn | — |
| **Agent Pet** | `crates/rice-maid-pet-rs` | Dịch vụ trợ lý pet đồng hành, hook `PreModelRequest` và tính toán bố cục | [agent-pet.md](agent-pet.md) |
| **DevKit CLI** | `crates/codex-plus-devkit` (`codex-plus-devkit`) | Tự động hóa phát hành (`release`, `release-meta`), `relaunch-local`, dọn bản build cũ (`prune-local-builds`), đồng bộ model (`sync-official-models`), `prepare-sherpa-cache` | [linux-packaging.md](linux-packaging.md) |

---

## 10. Các biện pháp tăng cường bảo mật đã áp dụng

Mọi thay đổi phát triển mới trong repo này cần duy trì các bất biến bảo mật sau:

1. **Chống Path Traversal & Symlink Escape**: Xác thực đường dẫn sau khi canonicalize cho việc xóa session, tách/gộp shard `renderer-inject` (cấm `..`, `/` hoặc `\`), và kiểm tra symlink/junction trước khi xử lý cache `sherpa-onnx`.
2. **An toàn WebView & XSS**: Toàn bộ dữ liệu HTML/SVG/URL trước khi render qua `renderer-inject-rs` hoặc Dioxus đều phải qua bộ lọc sanitize; không nhúng trực tiếp input chưa kiểm soát vào `dangerous_inner_html`.
3. **Chống Command Injection trên Windows (CVE-2024-24576)**: Không truyền tham số người dùng chưa lọc qua `cmd.exe /C`; luôn truyền mảng đối số tường minh tới `Command::new(...)` và dọn sạch biến môi trường nhạy cảm (`ANTHROPIC_*`, `CLAUDE_CODE_*`, `*_PROXY`) khi khởi chạy tiến trình con.
4. **Hàng rào Extension API**: Giữ cố định whitelist route mở rộng; tuyệt đối không mở route ghi cấu hình/xóa dữ liệu ra lớp `window.codexPlus`.
5. **Bí mật & Dữ liệu người dùng**: Không bao giờ ghi/commit `.env`, `auth.json`, `settings.json`, `state/`, hoặc token trong log lỗi.
