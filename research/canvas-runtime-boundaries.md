# `conversation-canvas` Runtime Boundaries (`ADR-006`)

Theo kiến trúc tách ranh giới `ADR-006` trong [`DECISIONS.md`](../DECISIONS.md), toàn bộ logic xử lý dữ liệu, chuẩn hóa tin nhắn, dựng/kiểm định đồ thị, bố cục cây 10.000 nút, đóng gói batch, giải mã SSE và dựng HTML thoát mã (`view-pages`, `bridge-data`, `model`, `tree-markup`, `tree-canvas`, `external-api`, `long-organizer`, `native-history`, `organize`) đều được port sang Rust/WASM (`crates/canvas-wasm/`).

Chỉ lớp gắn kết trình duyệt Electron bên trong tiến trình `ChatGPT.app` / Codex Desktop mới giữ dạng shim JS tối giản:

1. **`native-runtime.mjs`** (`42` dòng) — `blocked` (nhập động theo runtime Electron):
   - Thực thi dynamic ES module import `import("app://-/assets/app-initial-<hash>.js")` bên trong WebView Codex Desktop, tra bảng mapping tên export bị minify theo từng build (`f87238153a19`, `bcc2ff475eb6`, `f61fcec072b5`), và bọc thuộc tính động bằng `Object.defineProperty(..., { get: () => module[value] })` (vì live binding được Codex điền sau khi khởi động).
   - Không thể biểu diễn qua ranh giới JSON/WASM tĩnh vì phụ thuộc tham chiếu thực thể ES module V8 sống trong DOM host.
2. **`src/native-sidechat.js`, `src/native-api.js`, `src/canvas-store.js`, `src/tree-canvas-view.js`, `src/canvas-panel.js`**:
   - Duyệt cây `__reactFiber$` / `__reactContainer$` nội bộ của React, đọc `localStorage`/`IndexedDB` trực tiếp trong `app://-/index.html`, bắt sự kiện con trỏ/wheel/resize và gắn DOM với thuộc tính `[data-codex-plus-ext]`.
   - Phần dựng userscript (`build-userscript.mjs`) được port thuần Rust tại `canvas-core-rs::bundle_userscript`, và toàn bộ logic tính toán thuần của `conversation-canvas` (bao gồm các hàm bất đồng bộ/Map/Set `taskTreeView`, `layoutTaskCanvas`, `prepareHistory`, `mergeBatch`, `SseStreamDecoder`, `readNativeHistory`) đều nằm trong `crates/canvas-wasm/canvas-core-rs/`.
