# Codex++ Rust Port Workspace (`rust-port/`)

Thư mục này chứa toàn bộ cấu hình công cụ, dữ liệu phân tích (`rust-anything analyze`), bản đồ trạng thái (`MAP.md`), từ điển thuật ngữ (`GLOSSARY.md`), quyết định kiến trúc (`DECISIONS.md`) và các crate Rust được port từ các thành phần phi-Rust của dự án `CodexPlusPlus` bằng công cụ [`rust-anything`](https://github.com/erkamyaman/rust-anything) (`v3.4.0`).

> **Nguyên tắc cô lập**: Mọi mã nguồn gốc bên ngoài `rust-port/` được giữ nguyên làm nguồn đối chiếu hành vi (`record` / `diff` / `replay`). Không chỉnh sửa các tệp gốc cho tới khi đơn vị port vượt qua đầy đủ `rust-anything diff` và `rust-anything gate`.

## Công cụ đã thiết lập

- **CLI & MCP Server**: `rust-anything 3.4.0` (`~/.cargo/bin/rust-anything.exe`)
- **WASM Bindgen CLI**: `wasm-bindgen 0.2.130` (`~/.cargo/bin/wasm-bindgen.exe`)
- **Rust Toolchain**: `rustc 1.100.0` (`x86_64-pc-windows-gnu` + `wasm32-unknown-unknown`)
- **MCP cục bộ**: [`.mcp.json`](./.mcp.json) (`rust-anything mcp`, cung cấp 22 MCP tools)
- **Workflow Skills**: [`skills/rust-anything/SKILL.md`](./skills/rust-anything/SKILL.md) và [`skills/rust-anything/whole-app.md`](./skills/rust-anything/whole-app.md)

## Cấu trúc thư mục

```text
rust-port/
├── .mcp.json                 # Cấu hình MCP server cho coding agents
├── README.md                 # Tài liệu hướng dẫn workspace port
├── MAP.md                    # Bản đồ theo dõi tiến độ từng unit & wave
├── GLOSSARY.md               # Ánh xạ thuật ngữ domain & kiểu dữ liệu sang Rust
├── DECISIONS.md              # Các ADR kiến trúc (theo chuẩn whole-app.md)
├── skills/rust-anything/     # Bộ skill chuẩn của rust-anything
├── analysis/                 # Kết quả chạy `rust-anything analyze --whole-app` (.md & .json)
└── crates/                   # Các Rust unit crates & standalone binaries/apps được port
```

## Chu trình chuẩn cho mỗi Unit (`SKILL.md`)

```powershell
# 1. Tạo khung crate Rust + cầu nối (wasm hoặc napi/pyo3) vào rust-port/crates/<name>
rust-anything bridge <target-root> <unit-id> --kind wasm -o rust-port/crates/<crate-name>

# 2. Ghi nhận hành vi thực tế từ mã gốc + bộ test sẵn có
rust-anything record rust-port/crates/<crate-name> --from-tests -- npm test

# 3. Viết mã Rust trong src/port.rs, sau đó biên dịch & đối chiếu 100% cases
rust-anything build rust-port/crates/<crate-name>
rust-anything diff rust-port/crates/<crate-name>

# 4. Kiểm tra định nghĩa hoàn thành (fmt, clippy -D warnings, error coverage, 0 unwrap, diff N/N)
rust-anything gate rust-port/crates/<crate-name>
```
